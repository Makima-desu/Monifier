use select::{self, document::Document, predicate::{Class, Name}};
use futures::stream::{self, StreamExt};

use crate::models;
use crate::database;

#[tauri::command] // access the filter address and get the results by keyword
pub async fn search_manga(keywords: Vec<String>) -> Option<Vec<models::Manga>>
{
    let mut mangas: Vec<models::Manga> = Vec::new();

    let client = reqwest::Client::new();

    let keyword = keywords.join(" ");

    let url = format!("{}{}", models::FILTER, keyword);
    let response = client.get(&url).send().await.unwrap();

    if response.status().is_success()
    {
        let body = response.text().await.unwrap();
        
        let document = Document::from_read(body.as_bytes()).unwrap();
        
        for element in document.find(Class("info"))
        {
            let mut title: String = String::new(); 
            let mut href: String = String::new();
            let mut poster: String = String::new();
            let mut manga_type: String = String::new();
            let mut chapters: Vec<models::Chapter> = vec![];

            for title_element in element.find(Name("a"))
            {
                if title_element.parent().unwrap().name() != Some("li")
                {
                    title = title_element.text();
                    href = title_element.attr("href").unwrap().to_string();
                }

            }

            for manga_types in element.find(Class("type"))
            {
                manga_type = manga_types.text()
            }

            for content in element.find(Class("content"))
            {
                // go over parents attributes
                for attr in content.attrs()
                {
                    // if attribute is chapter get chapter
                    if attr.1 == "chap"
                    {
                        let mut href: String = String::new();
                        let attributes = content.text();
                        let parts: Vec<&str> = attributes.trim().split("     ").collect();
                        
                        for href_element in content.find(Name("a")).take(3)
                        {
                            href = href_element.attr("href").unwrap().to_string(); 
                        }
                        
                        for part in &parts
                        {
                            chapters.push(models::Chapter::get_chapter(part, &href))
    
                        }
    
                    }
                    // else if attr.1 == "vol" {}
                }
            }

            for image in element.parent().unwrap().find(Name("img"))
            {
                poster = image.attr("src").unwrap().to_string();
            }

            let manga: models::Manga = models::Manga
            {   
                id: None,
                title: title.clone(),
                chapters: Some(chapters.clone()),
                href: href.clone(),
                manga_type: models::MangaTypes::from_string_type(&manga_type),
                poster: Some(poster),
                favorited: models::Manga::is_favorited(title).unwrap()
            };

            mangas.push(manga)
        }
    }


    return Some(mangas)
}

// access the manga page and take the first chapter which will be used as latest
// pub async fn get_latest_chapter(href: &String) -> String
// {
//     let client = Client::new();
    
//     let mut chapter_number: String = String::new();
//     let response = client.get(format!("{}{}", models::BASE, href)).send().await.unwrap();

//     if response.status().is_success()
//     {
//         let body = response.text().await.unwrap();

//         let document = Document::from_read(body.as_bytes()).unwrap();

//         for element in document.find(Class("item")).take(1)
//         {
//             let chapter = element.text();
//             let chapter: Vec<&str> = chapter.trim().split(" ").collect();

//             // let date = chapter[3..].join(" ");
//             chapter_number = chapter[1].split(":").next().unwrap().to_string();
            
//         }
//     }

//     return chapter_number
// }

#[tauri::command]
pub async fn check_favorite_manga_parallel() -> Option<Vec<models::Manga>>
{
    let client = reqwest::Client::new();
    let manga_list = database::get_favorite_mangas();

    let mut mangas: Vec<models::Manga> = stream::iter(manga_list.unwrap())
    .map(|manga_item| 
    {
        let client = &client;
        async move 
        {
            let response = client.get(format!("{}{}", models::BASE, manga_item.href.clone())).send().await.unwrap();

            if response.status().is_success() 
            {
                if let Ok(body) = response.text().await 
                {
                    let document = Document::from_read(body.as_bytes()).ok();
                    return document.map(|doc| models::Manga::get_manga(doc, manga_item.href, manga_item.id.unwrap()));
                }
            }

            return None
        }
    })
    .buffer_unordered(10) // Adjust the concurrency level as needed
    .filter_map(|x| async { x })
    .collect().await;

    // sort mangas by last updated
    mangas.sort_by(|a, b| 
        {
            let date_a = a.chapters.as_ref().unwrap().first().map(|ch| models::Chapter::parse_date(&ch.date)).unwrap_or(chrono::Utc::now());
            let date_b = b.chapters.as_ref().unwrap().first().map(|ch| models::Chapter::parse_date(&ch.date)).unwrap_or(chrono::Utc::now());
            date_b.cmp(&date_a) // Note: cmp for reverse order (most recent first)
        });

    return Some(mangas)
}