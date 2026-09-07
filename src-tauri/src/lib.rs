mod commands;
mod crypto;
mod models;

use commands::vault::VaultState;
use tauri::Builder;

pub fn run() {
    Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .manage(VaultState::default())
        .invoke_handler(tauri::generate_handler![
            commands::vault::check_vault_exists,
            commands::vault::initialize_vault,
            commands::vault::unlock_vault,
            commands::vault::lock_vault,
            commands::vault::get_account_codes,
            commands::vault::add_account,
            commands::vault::delete_account,
            commands::vault::advance_hotp_counter,
            commands::system::copy_to_clipboard,
        ])
        .setup(|_app| Ok(()))
        .run(tauri::generate_context!())
        .expect("error while running authenticator application");
}