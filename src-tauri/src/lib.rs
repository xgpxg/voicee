mod commands;
mod utils;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            commands::gen_unique_id,
            commands::get_unique_id,
            commands::is_initialized,
            commands::update_verify_code,
            commands::download_model,
            commands::load_model,
            commands::voice_clone
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
