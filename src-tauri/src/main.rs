// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // Bundled apps launch with cwd "/", so the project root .env is unreachable.
    // dotenvy never overrides an already set var, so earlier files take precedence.
    if let Ok(home) = std::env::var("HOME") {
        let path = std::path::Path::new(&home).join(".config/dashboard/.env");
        match dotenvy::from_path(&path) {
            Ok(()) => eprintln!("Loaded .env from: {:?}", path),
            Err(e) => eprintln!("Failed to load {:?}: {:?}", path, e),
        }
    }

    // Project root .env for `npm run tauri dev`, which runs from src-tauri/
    let _ = dotenvy::from_filename("../.env");
    let _ = dotenvy::dotenv();

    // GUI apps get a minimal PATH without Homebrew or Docker Desktop binaries
    let path = std::env::var("PATH").unwrap_or_default();
    std::env::set_var("PATH", format!("/opt/homebrew/bin:/usr/local/bin:{path}"));

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
