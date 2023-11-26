// scraper for searching mangas

use reqwest::{self, Client};
use select::{self, document::Document, predicate::{Class, Name}};

use crate::models;
use crate::database;

#[tauri::command]
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
                title: title.clone(),
                chapters: Some(chapters.clone()),
                href: href.clone(),
                manga_type: models::MangaTypes::from_string_type(&manga_type),
                poster: Some(poster),
                latest_chapter: String::new(),
                favorited: models::Manga::is_favorited(title)
            };

            mangas.push(manga)
        }
    }

    return Some(mangas)
}

pub async fn get_latest_chapter(href: &String) -> String
{
    let client = Client::new();
    
    let mut chapter_number: String = String::new();
    let response = client.get(format!("{}{}", models::BASE, href)).send().await.unwrap();

    if response.status().is_success()
    {
        let body = response.text().await.unwrap();

        let document = Document::from_read(body.as_bytes()).unwrap();

        for element in document.find(Class("item")).take(1)
        {
            let chapter = element.text();
            let chapter: Vec<&str> = chapter.trim().split(" ").collect();

            // let date = chapter[3..].join(" ");
            chapter_number = chapter[1].split(":").next().unwrap().to_string();
            
        }
    }

    return chapter_number
}

#[tauri::command]
pub async fn check_update_manga_list() -> Option<Vec<models::Manga>>
{
    let manga_list = database::get_manga_list();
    let mut mangas: Vec<models::Manga> = Vec::new();

    let client = reqwest::Client::new();

    for mut manga in manga_list
    {
        let response = client.get(manga.href.clone()).send().await.unwrap();

        if response.status().is_success()
        {
            let body = response.text().await.unwrap();

            let document = Document::from_read(body.as_bytes()).unwrap();

            for element in document.find(Class("item")).take(1)
            {
                let chapter = element.text();
                let chapter: Vec<&str> = chapter.trim().split(" ").collect();

                // let date = chapter[3..].join(" ");
                let chapter_number = chapter[1].split(":").next().unwrap();
                let latest_chapter = manga.latest_chapter.parse::<u32>().unwrap();
                
                if manga.latest_chapter.is_empty()
                {
                    manga.latest_chapter = chapter_number.to_string();
                    database::add_manga_to_favorites(manga.clone());
                    mangas.push(manga.clone())
                }
                else if chapter_number.parse::<u32>().unwrap() > latest_chapter
                {
                    mangas.push(manga.clone())
                }
            }
        }
    }

    return Some(mangas)
}

#[tauri::command]
pub async fn get_recently_updated_manga(page_number: u32, manga_type: Option<models::MangaTypes>, genres: Option<Vec<models::Genres>>) -> Vec<models::Manga>
{
    let client = reqwest::Client::new();
    let response = client.get(format!("{}{}", models::UPDATES, page_number)).send().await.unwrap();

    let mut mangas: Vec<models::Manga> = vec![];

    if response.status().is_success()
    {
        let body = response.text().await.unwrap();

        let document = Document::from_read(body.as_bytes()).unwrap();

        for element in document.find(Class("info"))
        {
            mangas.push(models::Manga::get_manga(element));

        }
    }

    for manga in mangas
    {
        println!("{:#?}", manga)
    }

    return Vec::new();
}

// search genres / genre
// search author