#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    // if not string, try to convert to string
    format!("Hello {}!", name)
}

#[tauri::command]
fn my_custom_command(window: tauri::Window) -> String {
    format!("{}", window.inner_size())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, my_custom_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
