mod client;

use client::{FlorestaClient, FlorestadConfig};
use tauri::State;

#[tauri::command]
fn connect_node(
    config: FlorestadConfig,
    client: State<'_, FlorestaClient>,
) -> Result<String, String> {
    client.connect(config)?;
    Ok("Connected to florestad".to_string())
}

#[tauri::command]
fn disconnect_node(client: State<'_, FlorestaClient>) -> Result<String, String> {
    client.disconnect()?;
    Ok("Disconnected from florestad".to_string())
}

#[tauri::command]
fn get_node_config(client: State<'_, FlorestaClient>) -> Result<Option<FlorestadConfig>, String> {
    client.get_config()
}

#[tauri::command]
fn get_blockchain_info(client: State<'_, FlorestaClient>) -> Result<serde_json::Value, String> {
    let info = client.get_blockchain_info()?;
    serde_json::to_value(info).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_block_count(client: State<'_, FlorestaClient>) -> Result<u32, String> {
    client.get_block_count()
}

#[tauri::command]
fn get_best_block_hash(client: State<'_, FlorestaClient>) -> Result<String, String> {
    client.get_best_block_hash()
}

#[tauri::command]
fn get_block_hash(height: u32, client: State<'_, FlorestaClient>) -> Result<String, String> {
    client.get_block_hash(height)
}

#[tauri::command]
fn get_block_header(
    hash: String,
    client: State<'_, FlorestaClient>,
) -> Result<serde_json::Value, String> {
    client.get_block_header(hash)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .manage(FlorestaClient::new())
        .invoke_handler(tauri::generate_handler![
            connect_node,
            disconnect_node,
            get_node_config,
            get_blockchain_info,
            get_block_count,
            get_best_block_hash,
            get_block_hash,
            get_block_header,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
