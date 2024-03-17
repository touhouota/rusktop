// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

mod database;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskItem {
    title: String,
    description: String,
    estimate_sec: u64
}

impl TaskItem {
    pub fn new(title: &str, description: &str, estimate_sec: u64) -> Self {
        TaskItem {
            title: title.to_string(),
            description: description.to_string(),
            estimate_sec,
        }
    }
}

#[tauri::command]
fn append_task(title: &str, _description: &str, _estimate_sec: u64) -> Result<String, String> {
    dbg!(title);
    Ok(title.to_string())
}

#[tauri::command]
async fn show_tasks() -> Result<(), String> {
    let mut connect = database::open_db().unwrap();
    database::get_all_tasks(&connect);
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            append_task,
            show_tasks
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
