// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // Load .env file from project root
    match dotenvy::from_filename("../.env") {
        Ok(path) => eprintln!("Loaded .env from: {:?}", path),
        Err(e) => eprintln!("Failed to load .env: {:?}", e),
    }

    // Also try loading from current directory as fallback
    let _ = dotenvy::dotenv();

    eprintln!(
        "JIRA_API_TOKEN present: {}",
        std::env::var("JIRA_API_TOKEN").is_ok()
    );
    eprintln!(
        "JIRA_BOARD_ID present: {}",
        std::env::var("JIRA_BOARD_ID").is_ok()
    );

    dashboard_lib::run()
}
