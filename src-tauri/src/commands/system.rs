use tauri::AppHandle;
use tauri_plugin_clipboard_manager::ClipboardExt;

#[tauri::command]
pub async fn copy_to_clipboard(text: String, app: AppHandle) -> Result<(), String> {
    app.clipboard()
        .write_text(text)
        .map_err(|_| "Failed to write to system clipboard".to_string())?;

    // Spawn a background task to auto-clear the clipboard after 30 seconds for security
    tokio::spawn(async move {
        tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
        let _ = app.clipboard().clear();
    });

    Ok(())
}