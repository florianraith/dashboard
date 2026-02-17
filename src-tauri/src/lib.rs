use serde::Serialize;
use sysinfo::System;

#[derive(Serialize)]
struct ProcessInfo {
    name: String,
    memory: u64,
    percentage: f64,
}

#[derive(Serialize)]
struct RamUsage {
    used: u64,
    total: u64,
    percentage: f64,
    top_processes: Vec<ProcessInfo>,
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_ram_usage() -> RamUsage {
    let mut sys = System::new_all();
    sys.refresh_all();

    let total = sys.total_memory();
    let used = sys.used_memory();
    let percentage = if total > 0 {
        (used as f64 / total as f64) * 100.0
    } else {
        0.0
    };

    // Get all processes and sort by memory usage
    let mut processes: Vec<_> = sys
        .processes()
        .iter()
        .map(|(_, process)| {
            let memory = process.memory();
            let mem_percentage = if total > 0 {
                (memory as f64 / total as f64) * 100.0
            } else {
                0.0
            };

            ProcessInfo {
                name: process.name().to_string_lossy().to_string(),
                memory,
                percentage: mem_percentage,
            }
        })
        .collect();

    // Sort by memory usage (descending) and take top 5
    processes.sort_by(|a, b| b.memory.cmp(&a.memory));
    let top_processes = processes.into_iter().take(5).collect();

    RamUsage {
        used,
        total,
        percentage,
        top_processes,
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, get_ram_usage])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
