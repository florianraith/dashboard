use serde::Serialize;
use std::env;
use std::process::Command;
use sysinfo::System;
use tauri::{Manager, PhysicalPosition, PhysicalSize, Position, Size};

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

#[derive(Serialize)]
struct DockerContainer {
    id: String,
    name: String,
    image: String,
    status: String,
    ports: String,
    uptime: String,
}

#[derive(Serialize)]
struct SpotifyTrack {
    track_name: String,
    artist: String,
    album: String,
    artwork_url: String,
    is_playing: bool,
}

#[derive(Serialize)]
struct CpuCore {
    core_id: usize,
    usage: f32,
}

#[derive(Serialize)]
struct CpuProcessInfo {
    name: String,
    cpu_usage: f32,
}

#[derive(Serialize)]
struct CpuUsage {
    overall_usage: f32,
    cores: Vec<CpuCore>,
    top_processes: Vec<CpuProcessInfo>,
}

#[derive(Serialize)]
struct JiraTicket {
    key: String,
    summary: String,
    status: String,
    assignee: String,
    url: String,
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

fn parse_ports(ports_str: &str) -> String {
    if ports_str.is_empty() {
        return String::new();
    }

    // Parse port strings like "0.0.0.0:80->80/tcp, 0.0.0.0:3306->3306/tcp"
    // Extract just the host port numbers
    ports_str
        .split(',')
        .filter_map(|port| {
            let trimmed = port.trim();
            // Look for pattern like "0.0.0.0:80->80/tcp" or "80/tcp"
            if let Some(arrow_pos) = trimmed.find("->") {
                // Format: "0.0.0.0:80->80/tcp"
                let before_arrow = &trimmed[..arrow_pos];
                if let Some(colon_pos) = before_arrow.rfind(':') {
                    let port_num = &before_arrow[colon_pos + 1..];
                    return Some(port_num.to_string());
                }
            } else if trimmed.contains('/') {
                // Format: "80/tcp"
                if let Some(slash_pos) = trimmed.find('/') {
                    return Some(trimmed[..slash_pos].to_string());
                }
            }
            None
        })
        .collect::<Vec<_>>()
        .join(", ")
}

fn get_compose_info(container_id: &str) -> Option<(String, String)> {
    // Get labels from container
    let output = Command::new("docker")
        .args(&["inspect", "--format", "{{.Config.Labels}}", container_id])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let labels = String::from_utf8_lossy(&output.stdout);

    // Check if this is a compose container
    let mut project_name = None;
    let mut service_name = None;
    let mut working_dir = None;

    // Parse labels like: map[com.docker.compose.project:customer-portal com.docker.compose.service:mysql ...]
    for label in labels.split_whitespace() {
        if label.starts_with("com.docker.compose.project:") {
            project_name = Some(
                label
                    .trim_start_matches("com.docker.compose.project:")
                    .to_string(),
            );
        } else if label.starts_with("com.docker.compose.service:") {
            service_name = Some(
                label
                    .trim_start_matches("com.docker.compose.service:")
                    .to_string(),
            );
        } else if label.starts_with("com.docker.compose.project.working_dir:") {
            working_dir = Some(
                label
                    .trim_start_matches("com.docker.compose.project.working_dir:")
                    .to_string(),
            );
        }
    }

    // If we have both project and service, format the name
    if let (Some(service), Some(project)) = (service_name, project_name) {
        // Try to extract folder name from working_dir, otherwise use project name
        let folder_name = working_dir
            .and_then(|dir| {
                dir.trim_end_matches('/')
                    .split('/')
                    .last()
                    .map(|s| s.to_string())
            })
            .unwrap_or(project);

        return Some((folder_name, service));
    }

    None
}

#[tauri::command]
fn get_docker_containers() -> Result<Vec<DockerContainer>, String> {
    let output = Command::new("docker")
        .args(&[
            "ps",
            "--format",
            "{{.ID}}|{{.Names}}|{{.Image}}|{{.Status}}|{{.Ports}}|{{.RunningFor}}",
        ])
        .output()
        .map_err(|e| format!("Failed to execute docker command: {}", e))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let containers: Vec<DockerContainer> = stdout
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let parts: Vec<&str> = line.split('|').collect();
            let id = parts.get(0).unwrap_or(&"").to_string();
            let original_name = parts.get(1).unwrap_or(&"").to_string();
            let raw_ports = parts.get(4).unwrap_or(&"").to_string();

            // Check if this is a compose container and format name accordingly
            let display_name = if let Some((folder, service)) = get_compose_info(&id) {
                format!("{} - {}", folder, service)
            } else {
                original_name
            };

            // Parse and format ports
            let formatted_ports = parse_ports(&raw_ports);

            DockerContainer {
                id,
                name: display_name,
                image: parts.get(2).unwrap_or(&"").to_string(),
                status: parts.get(3).unwrap_or(&"").to_string(),
                ports: formatted_ports,
                uptime: parts.get(5).unwrap_or(&"").to_string(),
            }
        })
        .collect();

    Ok(containers)
}

#[tauri::command]
fn get_spotify_track() -> Result<SpotifyTrack, String> {
    #[cfg(target_os = "macos")]
    {
        let script = r#"
            tell application "Spotify"
                if it is running then
                    set trackName to name of current track
                    set artistName to artist of current track
                    set albumName to album of current track
                    set artworkUrl to artwork url of current track
                    set playerState to player state as string
                    return trackName & "|" & artistName & "|" & albumName & "|" & artworkUrl & "|" & playerState
                else
                    return "not_running"
                end if
            end tell
        "#;

        let output = Command::new("osascript")
            .arg("-e")
            .arg(script)
            .output()
            .map_err(|e| format!("Failed to execute AppleScript: {}", e))?;

        if !output.status.success() {
            return Err("Spotify is not running".to_string());
        }

        let result = String::from_utf8_lossy(&output.stdout).trim().to_string();

        if result == "not_running" {
            return Err("Spotify is not running".to_string());
        }

        let parts: Vec<&str> = result.split('|').collect();
        if parts.len() < 5 {
            return Err("Failed to parse Spotify data".to_string());
        }

        Ok(SpotifyTrack {
            track_name: parts[0].to_string(),
            artist: parts[1].to_string(),
            album: parts[2].to_string(),
            artwork_url: parts[3].to_string(),
            is_playing: parts[4] == "playing",
        })
    }

    #[cfg(not(target_os = "macos"))]
    {
        Err("Spotify integration is only supported on macOS".to_string())
    }
}

#[tauri::command]
fn get_cpu_usage() -> CpuUsage {
    let mut sys = System::new_all();

    // Need to refresh twice with a delay to get accurate CPU usage
    sys.refresh_cpu_all();
    std::thread::sleep(std::time::Duration::from_millis(200));
    sys.refresh_cpu_all();

    // Get per-core usage
    let cores: Vec<CpuCore> = sys
        .cpus()
        .iter()
        .enumerate()
        .map(|(index, cpu)| CpuCore {
            core_id: index,
            usage: cpu.cpu_usage(),
        })
        .collect();

    // Calculate overall CPU usage
    let overall_usage = if !cores.is_empty() {
        cores.iter().map(|c| c.usage).sum::<f32>() / cores.len() as f32
    } else {
        0.0
    };

    // Get top CPU-consuming processes
    let mut processes: Vec<_> = sys
        .processes()
        .iter()
        .map(|(_, process)| CpuProcessInfo {
            name: process.name().to_string_lossy().to_string(),
            cpu_usage: process.cpu_usage(),
        })
        .collect();

    // Sort by CPU usage (descending) and take top 5
    processes.sort_by(|a, b| b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap());
    let top_processes = processes.into_iter().take(5).collect();

    CpuUsage {
        overall_usage,
        cores,
        top_processes,
    }
}

#[tauri::command]
async fn get_jira_tickets() -> Result<Vec<JiraTicket>, String> {
    let api_token = env::var("JIRA_API_TOKEN")
        .map_err(|_| "JIRA_API_TOKEN environment variable not set".to_string())?;
    let email = env::var("JIRA_EMAIL")
        .map_err(|_| "JIRA_EMAIL environment variable not set".to_string())?;
    let base_url = env::var("JIRA_BASE_URL")
        .unwrap_or_else(|_| "https://zw-systems.atlassian.net".to_string());

    let raw_jql = env::var("JIRA_JQL").unwrap_or_default();
    let jql = if raw_jql.trim().is_empty() {
        "updated >= -3650d ORDER BY updated DESC".to_string()
    } else {
        raw_jql
    };

    let client = reqwest::Client::new();

    let auth_check = client
        .get(format!("{}/rest/api/3/myself", base_url))
        .basic_auth(&email, Some(&api_token))
        .header("Accept", "application/json")
        .send()
        .await
        .map_err(|e| format!("Failed to validate Jira credentials: {}", e))?;

    if !auth_check.status().is_success() {
        let status = auth_check.status();
        let body = auth_check.text().await.unwrap_or_default();
        return Err(format!(
            "Jira authentication failed ({}). Check JIRA_EMAIL and JIRA_API_TOKEN. {}",
            status, body
        ));
    }

    let url = format!(
        "{}/rest/api/3/search/jql?jql={}&maxResults=5&fields=summary,status,assignee",
        base_url,
        urlencoding::encode(&jql)
    );

    let response = client
        .get(&url)
        .basic_auth(&email, Some(&api_token))
        .header("Accept", "application/json")
        .send()
        .await
        .map_err(|e| format!("Failed to fetch Jira tickets: {}", e))?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("Jira API error ({}): {}", status, error_text));
    }

    let json: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse Jira response: {}", e))?;

    let issues = json["issues"]
        .as_array()
        .ok_or("Invalid Jira response format: missing 'issues' array")?;

    if issues.is_empty() {
        return Err(format!(
            "Jira returned 0 tickets for JQL: `{}`. Verify query and Jira permissions.",
            jql
        ));
    }

    let tickets: Vec<JiraTicket> = issues
        .iter()
        .map(|issue| {
            let key = issue["key"].as_str().unwrap_or("").to_string();
            let summary = issue["fields"]["summary"].as_str().unwrap_or("").to_string();
            let status = issue["fields"]["status"]["name"]
                .as_str()
                .unwrap_or("Unknown")
                .to_string();
            
            // Handle assignee which can be null
            let assignee = if issue["fields"]["assignee"].is_null() {
                "Unassigned".to_string()
            } else {
                issue["fields"]["assignee"]["displayName"]
                    .as_str()
                    .unwrap_or("Unassigned")
                    .to_string()
            };
            
            let url = format!("{}/browse/{}", base_url, key);

            JiraTicket {
                key,
                summary,
                status,
                assignee,
                url,
            }
        })
        .collect();

    Ok(tickets)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            if let Some(window) = app.get_webview_window("main") {
                let monitors = window.available_monitors()?;
                let target_monitor = monitors.get(1).or_else(|| monitors.first());

                if let Some(monitor) = target_monitor {
                    let position = monitor.position();
                    let size = monitor.size();

                    window.set_position(Position::Physical(PhysicalPosition::new(
                        position.x,
                        position.y,
                    )))?;

                    window.set_size(Size::Physical(PhysicalSize::new(size.width, size.height)))?;
                }
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            get_ram_usage,
            get_docker_containers,
            get_spotify_track,
            get_cpu_usage,
            get_jira_tickets
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
