<img width="1512" height="982" alt="image" src="https://github.com/user-attachments/assets/790829c3-bac2-44b7-80fb-073e67e5de1b" />

# Dashboard

Tauri 2 desktop dashboard for macOS. Shows RAM, CPU, Docker containers, Spotify now playing, service health checks, Sentry issues and Jira tickets.

## Requirements

- macOS (Spotify and window placement rely on macOS APIs)
- Xcode Command Line Tools: `xcode-select --install`
- Rust (stable) via [rustup](https://rustup.rs): `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Node.js 20+ and npm, e.g. `brew install node`
- Optional: Docker Desktop (Docker widget) and Spotify (Spotify widget)

## Setup

```bash
git clone https://github.com/florianraith/dashboard.git
cd dashboard
npm install
```

### Configuration

All integrations are configured through a `.env` file. Start from the template:

```bash
cp .env.example .env
```

| Variable | Description |
| --- | --- |
| `JIRA_EMAIL` | Jira account email |
| `JIRA_API_TOKEN` | Token from https://id.atlassian.com/manage-profile/security/api-tokens |
| `JIRA_BASE_URL` | e.g. `https://your-domain.atlassian.net` |
| `JIRA_JQL` | Optional. Defaults to `updated >= -3650d ORDER BY updated DESC` |
| `SENTRY_AUTH_TOKEN` | Sentry token with `event:read` scope |
| `SENTRY_ORG` | Organization slug |
| `SENTRY_PROJECT_ID` | Numeric project ID |
| `SENTRY_PATH_PREFIX` | Optional. Server path stripped from issue titles |
| `HEALTH_CHECKS` | Comma separated `Name\|URL` pairs, e.g. `'App\|https://app.example.com/up,API\|https://api.example.com/health'` |

Widgets whose variables are missing show an error instead of data. The rest of the app keeps working.

The app looks for `.env` in this order (earlier files win):

1. `~/.config/dashboard/.env`
2. `.env` in the project root (only found during `npm run tauri dev`)

The installed app starts with working directory `/`, so it only sees `~/.config/dashboard/.env`:

```bash
mkdir -p ~/.config/dashboard
cp .env ~/.config/dashboard/.env
```

## Development

```bash
npm run tauri dev
```

Builds the Rust backend, starts Vite on port 1420 and opens the window with hot reload. The first run compiles all Rust dependencies and takes a few minutes.

```bash
npm run check          # svelte-check + TypeScript
cd src-tauri && cargo check
```

`npm run dev` starts only the frontend in a browser. Widgets fail there because Tauri IPC is unavailable.

## Production build and install

```bash
npm run tauri build
cp -R src-tauri/target/release/bundle/macos/Dashboard.app /Applications/
```

Then start Dashboard from Launchpad or Spotlight. To update, rebuild and copy again (quit the running app first).

The build is unsigned. If macOS blocks it on first launch, right click the app in Finder and choose Open, or run `xattr -cr /Applications/Dashboard.app`.

### Start at login

System Settings > General > Login Items > add Dashboard.

## Usage notes

- The window opens on the second monitor and fills its usable area. Without a second monitor it uses the main one. Change this in `src-tauri/src/lib.rs`.
- The window has no title bar. Drag it by the thin strip along its top edge. Quit with `Cmd+Q`.
- On first Spotify poll macOS asks whether Dashboard may control Spotify. Allow it, or enable it later under System Settings > Privacy & Security > Automation.
- The Docker widget calls the `docker` CLI. It must be in `/opt/homebrew/bin`, `/usr/local/bin` or the default `PATH`, and Docker Desktop must be running.
