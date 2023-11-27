use select::document::Document;
use serde::ser::{Serialize, Serializer};
use chrono::{self, DateTime, Duration, Utc, NaiveDate};

pub const BASE: &str = "https://mangafire.to";
pub const FILTER: &str = "https://mangafire.to/filter?keyword=";
pub const _UPDATES: &str = "https://mangafire.to/updated?page=";
pub const _RANDOM: &str = "https://mangafire.to/random";

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

pub type _CommandResult<T, E = CommandError> = anyhow::Result<T, E>;

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

    pub fn parse_date(date_str: &String) -> DateTime<Utc>
    {
        if let Some(seconds) = date_str.strip_suffix(" seconds ago") 
        {
            let seconds = seconds.parse::<i64>().unwrap_or(0);
            Utc::now() - Duration::seconds(seconds)
        }
        else if let Some(hours) = date_str.strip_suffix(" hours ago") 
        {
            let hours = hours.parse::<i64>().unwrap_or(0);
            Utc::now() - Duration::hours(hours)
        }
        else 
        {
            NaiveDate::parse_from_str(date_str.to_string().as_str(), "%b %d, %Y")
                .map(|d| DateTime::<Utc>::from_naive_utc_and_offset(d.and_hms_opt(0, 0, 0).unwrap(), Utc))
                .unwrap_or_else(|_| Utc::now())
        }
    }
}

#[derive(Debug, Clone)]
#[derive(Eq, Hash, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Manga
{
    pub id: Option<u64>,
    pub title: String,
    pub manga_type: Option<MangaTypes>,
    pub chapters: Option<Vec<Chapter>>,
    pub href: String,
    pub poster: Option<String>,
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

    pub fn get_manga(document: Document, url: String, id: u64) -> Self
    {
        use select::predicate::{Name, Class};

        let mut manga_type = String::new(); // get manga type
        let mut title = String::new(); // manga title
        let mut poster: String = String::new();

        let mut chapters: Vec<Chapter> = Vec::new(); // chapters of the manga
        let mut latest_chapter: String = String::new();

        // get title in <h1> 
        for element in document.find(Name("h1"))
        {
            title = element.text();

        }

        for type_element in document.find(Class("min-info")).take(1)
        {
            manga_type = type_element.text().trim().split_once(" ").unwrap().0.to_string()

        }

        for image in document.find(Name("img"))
        {
            if image.attr("itemprop") == Some("image")
            {
                poster = image.attr("src").unwrap().to_string()
            }
        }

        // get the first three chapters
        for item in document.find(Class("item")).take(3)
        {
            let chapter = item.text();
            let chapter: Vec<&str> = chapter.trim().split(" ").collect();
            let mut chapter_link: String = String::new();

            let chapter_number = chapter[1].split(":").next().unwrap();
            let date = chapter[&chapter.len() - 3..].join(" ");

            if latest_chapter.is_empty() { latest_chapter = chapter_number.to_string() }
            
            for link in item.find(Name("a"))
            {
                chapter_link = link.attr("href").unwrap().to_string()
            }

            let chapter: Chapter = Chapter
            {
                chapter_number: chapter_number.to_string(),
                chapter_link: chapter_link,
                date: date,
                language: String::from("EN")
            };

            chapters.push(chapter)

        }
        
        return Self {
            id: Some(id),
            title: title,
            href: url,
            chapters: Some(chapters),
            poster: Some(poster),
            manga_type: MangaTypes::from_string_type(&manga_type),
            favorited: false
        };

    }
}
