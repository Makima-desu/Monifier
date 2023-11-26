use std::path::Path;
use rusqlite::Connection;
use crate::models;

const DB_PATH: &str = "./manga.db";

pub fn create_database() -> Result<(), rusqlite::Error>
{
    if !Path::new(DB_PATH).exists()
    {
        let conn = Connection::open("./manga.db")?;

        conn.execute("create table favorites (id INTEGER PRIMARY KEY, title TEXT, href TEXT)", [])?;
    }

    Ok(())
}

pub fn get_manga_list() -> Vec<models::Manga>
{
    let conn = Connection::open(DB_PATH).unwrap();

    let mut statement = conn.prepare("select * from favorites").unwrap();

    let mangas = statement.query_map([], |row|
    {
        Ok(models::Manga
        {
            title: row.get(1)?,
            chapters: None,
            href: row.get(2)?,
            poster: None,
            manga_type: None,
            latest_chapter: row.get(3)?,
            favorited: true,
        })
    });

    let mut mangas_vec: Vec<models::Manga> = Vec::new();

    for manga in mangas.unwrap()
    {
        mangas_vec.push(manga.unwrap())
    }

    return mangas_vec
}

#[tauri::command]
pub async fn add_manga_to_favorites(manga: models::Manga)
{
    use crate::scraper;
    let conn = Connection::open(DB_PATH).unwrap();

    let latest_chapter = scraper::get_latest_chapter(&manga.href).await;

    let _ = conn.execute("insert into favorites (title, href, latest_chapter) values (?1, ?2, ?3)", [&manga.title, &manga.href, &latest_chapter]).unwrap();
}