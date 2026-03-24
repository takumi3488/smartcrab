#![deny(clippy::dbg_macro, clippy::expect_used, clippy::unwrap_used)]
#![warn(clippy::pedantic)]

pub mod commands;
pub mod db;
pub mod error;

use db::DbState;

/// Build and run the Tauri application.
///
/// # Errors
///
/// Returns an error if Tauri fails to initialise or the database cannot be
/// opened.
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "smartcrab.db";
    let db_state = DbState::open(db_path)?;

    tauri::Builder::default()
        .manage(db_state)
        .invoke_handler(tauri::generate_handler![
            commands::execution::execute_pipeline,
            commands::execution::cancel_execution,
            commands::execution::get_execution_history,
            commands::execution::get_execution_detail,
        ])
        .run(tauri::generate_context!())
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

    Ok(())
}
