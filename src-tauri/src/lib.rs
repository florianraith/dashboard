use serde::Serialize;
use std::env;
use std::process::Command;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{DateTime, Utc};
use sysinfo::System;
use tauri::{AppHandle, Manager, PhysicalPosition, PhysicalSize, Position, Size, State};

#[derive(Clone, Serialize)]
struct ProcessInfo {
    name: String,
    memory: u64,
    percentage: f64,
}

#[derive(Clone, Serialize)]
struct RamUsage {
    used: u64,
    total: u64,
    percentage: f64,
    top_processes: Vec<ProcessInfo>,
}

#[derive(Clone, Serialize)]
struct DockerContainer {
    id: String,
    name: String,
    image: String,
    status: String,
    ports: String,
    uptime: String,
}

#[derive(Clone, Serialize)]
struct SpotifyTrack {
    track_name: String,
    artist: String,
    album: String,
    artwork_url: String,
    is_playing: bool,
}

#[derive(Clone, Serialize)]
struct CpuCore {
    core_id: usize,
    usage: f32,
}

#[derive(Clone, Serialize)]
struct CpuProcessInfo {
    name: String,
    cpu_usage: f32,
}

#[derive(Clone, Serialize)]
struct CpuUsage {
    overall_usage: f32,
    cores: Vec<CpuCore>,
    top_processes: Vec<CpuProcessInfo>,
}

#[derive(Clone, Serialize)]
struct JiraTicket {
    key: String,
    summary: String,
    status: String,
    assignee: String,
    url: String,
}

#[derive(Clone, Serialize)]
struct ServiceHealth {
    name: String,
    url: String,
    is_up: bool,
    status_code: Option<u16>,
    latency_ms: Option<u128>,
    checked_at_ms: u128,
    error: Option<String>,
}

#[derive(Clone, Serialize)]
struct SentryIssue {
    title: String,
    last_seen: String,
    first_seen: String,
    age: String,
    events: u64,
    users: u64,
    is_bot: bool,
    url: String,
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn collect_ram_usage() -> RamUsage {
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

    // Sort by memory usage (descending) and take top 3
    processes.sort_by(|a, b| b.memory.cmp(&a.memory));
    let top_processes = processes.into_iter().take(3).collect();

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

fn collect_docker_containers() -> Result<Vec<DockerContainer>, String> {
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

fn collect_spotify_track() -> Result<SpotifyTrack, String> {
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

fn collect_cpu_usage() -> CpuUsage {
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

    // Sort by CPU usage (descending) and take top 3
    processes.sort_by(|a, b| b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap());
    let top_processes = processes.into_iter().take(3).collect();

    CpuUsage {
        overall_usage,
        cores,
        top_processes,
    }
}

async fn collect_jira_tickets() -> Result<Vec<JiraTicket>, String> {
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
        "{}/rest/api/3/search/jql?jql={}&maxResults=15&fields=summary,status,assignee",
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

async fn collect_service_health() -> Vec<ServiceHealth> {
    let services = [
        ("Trisolaris", "https://app.florianraith.com/up"),
        ("Spliit", "https://spliit.florianraith.com/api/health"),
        ("Partnerportal (Dev)", "https://dev-portal.zewotherm.com/up"),
        ("Partnerportal (Prod)", "https://portal.zewotherm.com/up"),
    ];

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(8))
        .build()
        .unwrap_or_else(|_| reqwest::Client::new());

    let mut results = Vec::with_capacity(services.len());

    for (name, url) in services {
        let checked_at_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis())
            .unwrap_or(0);
        let started = std::time::Instant::now();
        let response = client.get(url).send().await;

        match response {
            Ok(resp) => {
                let status = resp.status();
                results.push(ServiceHealth {
                    name: name.to_string(),
                    url: url.to_string(),
                    is_up: status.is_success(),
                    status_code: Some(status.as_u16()),
                    latency_ms: Some(started.elapsed().as_millis()),
                    checked_at_ms,
                    error: None,
                });
            }
            Err(err) => {
                results.push(ServiceHealth {
                    name: name.to_string(),
                    url: url.to_string(),
                    is_up: false,
                    status_code: None,
                    latency_ms: Some(started.elapsed().as_millis()),
                    checked_at_ms,
                    error: Some(err.to_string()),
                });
            }
        }
    }

    results
}

fn format_age_from_first_seen(first_seen: &str) -> String {
    let parsed = DateTime::parse_from_rfc3339(first_seen);
    if let Ok(first_seen_dt) = parsed {
        let now = Utc::now();
        let first_seen_utc = first_seen_dt.with_timezone(&Utc);
        let delta = now.signed_duration_since(first_seen_utc);

        if delta.num_days() > 0 {
            return format!("{}d", delta.num_days());
        }
        if delta.num_hours() > 0 {
            return format!("{}h", delta.num_hours());
        }
        if delta.num_minutes() > 0 {
            return format!("{}m", delta.num_minutes());
        }
        return format!("{}s", delta.num_seconds().max(0));
    }

    "n/a".to_string()
}

async fn collect_sentry_issues() -> Result<Vec<SentryIssue>, String> {
    let token = env::var("SENTRY_AUTH_TOKEN")
        .map_err(|_| "SENTRY_AUTH_TOKEN environment variable not set".to_string())?;

    let url = "https://sentry.io/api/0/organizations/zw-systems-gmbh/issues/?project=4509966802485248&statsPeriod=90d&sort=date&limit=15&query=is:unresolved";

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(12))
        .build()
        .unwrap_or_else(|_| reqwest::Client::new());

    let response = client
        .get(url)
        .header("Authorization", format!("Bearer {}", token))
        .header("Accept", "application/json")
        .send()
        .await
        .map_err(|e| format!("Failed to fetch Sentry issues: {}", e))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(format!("Sentry API error ({}): {}", status, body));
    }

    let json: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse Sentry response: {}", e))?;

    let issues = json
        .as_array()
        .ok_or("Invalid Sentry response format: expected array")?;

    let mapped = issues
        .iter()
        .map(|issue| {
            let title = issue["title"]
                .as_str()
                .or_else(|| issue["metadata"]["title"].as_str())
                .unwrap_or("Unknown issue")
                .to_string();

            let last_seen = issue["lastSeen"]
                .as_str()
                .unwrap_or("n/a")
                .to_string();
            let first_seen = issue["firstSeen"]
                .as_str()
                .unwrap_or("n/a")
                .to_string();

            let events = issue["count"]
                .as_str()
                .and_then(|v| v.parse::<u64>().ok())
                .or_else(|| issue["count"].as_u64())
                .unwrap_or(0);

            let users = issue["userCount"].as_u64().unwrap_or(0);
            let is_bot = issue["tags"]
                .as_array()
                .map(|tags| {
                    tags.iter().any(|tag| {
                        tag["key"].as_str() == Some("browser")
                            && tag["value"]
                                .as_str()
                                .map(|value| value.contains("Python"))
                                .unwrap_or(false)
                    })
                })
                .unwrap_or(false);
            let url = issue["permalink"].as_str().unwrap_or("").to_string();

            SentryIssue {
                title,
                age: format_age_from_first_seen(&first_seen),
                last_seen,
                first_seen,
                events,
                users,
                is_bot,
                url,
            }
        })
        .collect();

    Ok(mapped)
}

#[tauri::command]
fn get_ram_usage(state: State<'_, AppState>) -> RamUsage {
    state
        .snapshot
        .read()
        .expect("failed to lock state")
        .ram
        .clone()
}

#[tauri::command]
fn get_cpu_usage(state: State<'_, AppState>) -> CpuUsage {
    state
        .snapshot
        .read()
        .expect("failed to lock state")
        .cpu
        .clone()
}

#[tauri::command]
fn get_docker_containers(state: State<'_, AppState>) -> Result<Vec<DockerContainer>, String> {
    state
        .snapshot
        .read()
        .expect("failed to lock state")
        .docker
        .clone()
}

#[tauri::command]
fn get_spotify_track(state: State<'_, AppState>) -> Result<SpotifyTrack, String> {
    state
        .snapshot
        .read()
        .expect("failed to lock state")
        .spotify
        .clone()
}

#[tauri::command]
fn get_jira_tickets(state: State<'_, AppState>) -> Result<Vec<JiraTicket>, String> {
    state
        .snapshot
        .read()
        .expect("failed to lock state")
        .jira
        .clone()
}

#[tauri::command]
fn get_service_health(state: State<'_, AppState>) -> Vec<ServiceHealth> {
    state
        .snapshot
        .read()
        .expect("failed to lock state")
        .health
        .clone()
}

#[tauri::command]
fn get_sentry_issues(state: State<'_, AppState>) -> Result<Vec<SentryIssue>, String> {
    state
        .snapshot
        .read()
        .expect("failed to lock state")
        .sentry
        .clone()
}

fn start_background_pollers(app: &AppHandle) {
    let snapshot_for_ram = app.state::<AppState>().snapshot.clone();
    tauri::async_runtime::spawn(async move {
        tokio::time::sleep(Duration::from_millis(100)).await;
        loop {
            if let Ok(ram) = tauri::async_runtime::spawn_blocking(collect_ram_usage).await {
                if let Ok(mut state) = snapshot_for_ram.write() {
                    state.ram = ram;
                }
            }
            tokio::time::sleep(Duration::from_millis(2000)).await;
        }
    });

    let snapshot_for_cpu = app.state::<AppState>().snapshot.clone();
    tauri::async_runtime::spawn(async move {
        tokio::time::sleep(Duration::from_millis(450)).await;
        loop {
            if let Ok(cpu) = tauri::async_runtime::spawn_blocking(collect_cpu_usage).await {
                if let Ok(mut state) = snapshot_for_cpu.write() {
                    state.cpu = cpu;
                }
            }
            tokio::time::sleep(Duration::from_millis(2000)).await;
        }
    });

    let snapshot_for_docker = app.state::<AppState>().snapshot.clone();
    tauri::async_runtime::spawn(async move {
        tokio::time::sleep(Duration::from_millis(850)).await;
        loop {
            if let Ok(docker) = tauri::async_runtime::spawn_blocking(collect_docker_containers).await {
                if let Ok(mut state) = snapshot_for_docker.write() {
                    state.docker = docker;
                }
            }
            tokio::time::sleep(Duration::from_millis(5000)).await;
        }
    });

    let snapshot_for_spotify = app.state::<AppState>().snapshot.clone();
    tauri::async_runtime::spawn(async move {
        tokio::time::sleep(Duration::from_millis(1250)).await;
        loop {
            if let Ok(spotify) = tauri::async_runtime::spawn_blocking(collect_spotify_track).await {
                if let Ok(mut state) = snapshot_for_spotify.write() {
                    state.spotify = spotify;
                }
            }
            tokio::time::sleep(Duration::from_millis(3000)).await;
        }
    });

    let snapshot_for_jira = app.state::<AppState>().snapshot.clone();
    tauri::async_runtime::spawn(async move {
        tokio::time::sleep(Duration::from_millis(1700)).await;
        loop {
            let jira = collect_jira_tickets().await;
            if let Ok(mut state) = snapshot_for_jira.write() {
                state.jira = jira;
            }
            tokio::time::sleep(Duration::from_millis(30000)).await;
        }
    });

    let snapshot_for_health = app.state::<AppState>().snapshot.clone();
    tauri::async_runtime::spawn(async move {
        tokio::time::sleep(Duration::from_millis(2100)).await;
        loop {
            let health = collect_service_health().await;
            if let Ok(mut state) = snapshot_for_health.write() {
                state.health = health;
            }
            tokio::time::sleep(Duration::from_millis(20000)).await;
        }
    });

    let snapshot_for_sentry = app.state::<AppState>().snapshot.clone();
    tauri::async_runtime::spawn(async move {
        tokio::time::sleep(Duration::from_millis(2500)).await;
        loop {
            let sentry = collect_sentry_issues().await;
            if let Ok(mut state) = snapshot_for_sentry.write() {
                state.sentry = sentry;
            }
            tokio::time::sleep(Duration::from_millis(30000)).await;
        }
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app_state = AppState::new();

    tauri::Builder::default()
        .manage(app_state)
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            start_background_pollers(app.handle());

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
            get_jira_tickets,
            get_service_health,
            get_sentry_issues
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
#[derive(Clone)]
struct AppSnapshot {
    ram: RamUsage,
    cpu: CpuUsage,
    docker: Result<Vec<DockerContainer>, String>,
    spotify: Result<SpotifyTrack, String>,
    jira: Result<Vec<JiraTicket>, String>,
    health: Vec<ServiceHealth>,
    sentry: Result<Vec<SentryIssue>, String>,
}

struct AppState {
    snapshot: Arc<RwLock<AppSnapshot>>,
}

impl AppState {
    fn new() -> Self {
        Self {
            snapshot: Arc::new(RwLock::new(AppSnapshot {
                ram: RamUsage {
                    used: 0,
                    total: 0,
                    percentage: 0.0,
                    top_processes: Vec::new(),
                },
                cpu: CpuUsage {
                    overall_usage: 0.0,
                    cores: Vec::new(),
                    top_processes: Vec::new(),
                },
                docker: Ok(Vec::new()),
                spotify: Err("Loading Spotify data...".to_string()),
                jira: Err("Loading Jira tickets...".to_string()),
                health: Vec::new(),
                sentry: Err("Loading Sentry issues...".to_string()),
            })),
        }
    }
}
