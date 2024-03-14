// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use serde::{Deserialize, Serialize};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

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

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            append_task,
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
