// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

mod database;
use database::Task;

use std::thread::current;
use serde::{Deserialize, Serialize};

#[tauri::command]
fn append_task(title: &str, _explain: &str, _estimate_sec: u64) -> Result<String, String> {
    dbg!(title);
    Ok(title.to_string())
}

#[tauri::command]
async fn get_tasks() -> Result<Vec<Task>, String> {
    let connect = database::open_db().unwrap();
    let tasks = database::get_tasks(&connect);
    Ok(tasks)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            append_task,
            get_tasks
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
