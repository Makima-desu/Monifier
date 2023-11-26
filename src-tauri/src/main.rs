// Prevents additional console window on Windows in release, DO NOT REMOVE!!
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod scraper;
mod database;
mod models;

fn main() 
{
    let _ = database::create_database();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            scraper::search_manga,
            scraper::check_update_manga_list,
            scraper::get_recently_updated_manga,
            database::add_manga_to_favorites
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
