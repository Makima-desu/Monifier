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

//
pub fn get_favorite_mangas() -> Result<Vec<models::Manga>, rusqlite::Error>
{
    let conn = Connection::open(DB_PATH)?;

    let mut statement = conn.prepare("select * from favorites")?;

    let mangas = statement.query_map([], |row|
    {
        Ok(models::Manga
        {
            id: row.get(0)?,
            title: row.get(1)?,
            chapters: None,
            href: row.get(2)?,
            poster: None,
            manga_type: None,
            favorited: true,
        })
    });

    let mut mangas_vec: Vec<models::Manga> = Vec::new();

    for manga in mangas.unwrap()
    {
        mangas_vec.push(manga.unwrap())
    }

    return Ok(mangas_vec)
}

#[tauri::command]
pub async fn add_manga_to_favorites(manga: models::Manga)
{
    let conn = Connection::open(DB_PATH).unwrap();

    let _ = conn.execute("insert into favorites (title, href) values (?1, ?2)", [&manga.title, &manga.href]).unwrap();
}

#[tauri::command]
pub fn remove_from_favorites(id: u64)
{
    let conn = Connection::open(DB_PATH).unwrap();

    let _ = conn.execute("DELETE from favorites where id = ?", [&id]).unwrap();

}