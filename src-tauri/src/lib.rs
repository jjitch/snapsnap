mod snap;
mod types;

#[tauri::command]
async fn one_time_shoot() -> tauri::ipc::Response {
    let img = snap::snap();
    tauri::ipc::Response::new(img)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![one_time_shoot])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
