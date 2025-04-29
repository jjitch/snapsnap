#[tauri::command]
async fn one_time_shoot() -> tauri::ipc::Response {
    let snap = snap::snap();
    let _ts = snap.0;
    let img = snap.1;
    tauri::ipc::Response::new(img)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![one_time_shoot])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub mod snap;
pub mod types;
