use serde::ser::{Serialize, Serializer};

pub const BASE: &str = "https://mangafire.to";
pub const FILTER: &str = "https://mangafire.to/filter?keyword=";
pub const UPDATES: &str = "https://mangafire.to/updated?page=";
pub const RANDOM: &str = "https://mangafire.to/random";

// error catching
#[derive(Debug, thiserror::Error)]
pub enum CommandError
{
    #[error(transparent)]
    Rusqlite(#[from] rusqlite::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error)
}

// implement serialize for custom errors
impl Serialize for CommandError
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer 
    {
        serializer.serialize_str(self.to_string().as_ref())

    }

}

pub type CommandResult<T, E = CommandError> = anyhow::Result<T, E>;

#[derive(Debug, Clone)]
#[derive(Eq, Hash, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum MangaTypes
{
    Manga,
    Manhua,
    Manhwa,
    Novel,
    OneShot,
    Doujinshi
}

impl MangaTypes
{
    pub fn from_string_type(manga_type: &String) -> Option<MangaTypes>
    {
        match manga_type.to_lowercase().as_str()
        {
            "manga" => Some(MangaTypes::Manga),
            "manhua" => Some(MangaTypes::Manhua),
            "manhwa" => Some(MangaTypes::Manhwa),
            "novel" => Some(MangaTypes::Novel),
            "oneshot" => Some(MangaTypes::OneShot),
            "doujinshi" => Some(MangaTypes::Doujinshi),
            _ => None
        }
    }
}

#[derive(Debug, Clone)]
#[derive(Eq, Hash, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum Status
{
    Completed,
    Releasing,
    Hiatus,
    Discontinued,
    Unpuplished
}

#[derive(Debug, Clone)]
#[derive(Eq, Hash, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum Genres
{
    Action,
    Comedy,
    Fantasy,
    Horror,
    Kids,
    Mecha,
    Parody,
    School,
    Shounen,
    SuperPower,
    Vampire,
    Adventure,
    Demons,
    GirlsLove,
    Isekai,
    Magic,
    Military,
    Psychological,
    SciFi,
    SliceOfLife,
    Supernatural,
    AvantGarde,
    Drama,
    Gourmet,
    Iyashikei,
    MahouShoujo,
    Music,
    ReverseHarem,
    Seinen,
    Space,
    Suspense,
    BoysLove,
    Ecchi,
    Harem,
    Josei,
    MartialArts,
    Mystery,
    Romance,
    Shoujo,
    Sports,
    Thriller,
}

#[derive(Debug, Clone)]
#[derive(Eq, Hash, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Chapter
{
    pub chapter_number: String,
    pub language: String,
    pub chapter_link: String,
    pub date: String,
}

impl Chapter
{
    pub fn get_chapter(part: &str, href: &String) -> Chapter
    {
        let chapter_data: Vec<&str> = part.trim().split_whitespace().collect();
        
        let chapter: Chapter = Chapter
        {
            chapter_number: chapter_data[1].to_string(),
            language: chapter_data[2].to_string(),
            chapter_link: href.to_string(),
            date: chapter_data[3..].join(" ")
        };

        return chapter

    }
}

#[derive(Debug, Clone)]
#[derive(Eq, Hash, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Manga
{
    pub title: String,
    pub manga_type: Option<MangaTypes>,
    pub chapters: Option<Vec<Chapter>>,
    pub href: String,
    pub poster: Option<String>,
    pub latest_chapter: String,
    pub favorited: bool,

}

impl Manga
{
    pub fn is_favorited(title: String) -> bool
    {
        use crate::database;

        let mangas: Vec<Manga> = database::get_manga_list();

        for manga in mangas
        {
            if manga.title == title
            {
                return true
            }
        }

        return false
    }

    pub fn get_manga(element: select::node::Node) -> Manga
    {
        use select::predicate::{Name, Class};

        let manga_type = element.text(); // get manga type
        let mut title = String::new(); // manga title
        let mut href: String = String::new(); // manga page link

        let mut chapters: Vec<Chapter> = Vec::new(); // chapters of the manga

        // iterate over info class parent element and find all <a> tags
        for title_element in element.find(Name("a"))
        {
            // get manga title and link to manga page
            if title_element.parent().unwrap().name() != Some("li")
            {
                title = title_element.text();
                href = title_element.attr("href").unwrap().to_string();

            }
        }

        // content refers to chapters or volumes if applicable
        for content in element.find(Class("content"))
        {
            // go over parents attributes
            for attr in content.attrs()
            {
                // if attribute is chapter get chapter
                if attr.1 == "chap"
                {
                    
                    let attributes = content.text();
                    let parts: Vec<&str> = attributes.trim().split("     ").collect();
                    
                    for part in parts
                    {
                        // chapters.push(Chapter::get_chapter(part))

                    }

                }
                // else if attr.1 == "vol" {}
            }
        }
        
        
        let manga: Manga = Manga
        {
            title: title,
            href: href,
            chapters: Some(chapters.clone()),
            poster: None,
            manga_type: None,
            latest_chapter: chapters[0].chapter_number.clone(),
            favorited: false
        };

        return manga
    }
}
