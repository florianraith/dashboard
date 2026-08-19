# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

A personal Tauri 2 desktop dashboard (macOS-oriented) that fills a monitor with widgets: RAM, CPU, Docker containers, Spotify now-playing, service health checks, Sentry issues, Jira tickets. Frontend is SvelteKit 5 in SPA mode; all data collection happens in Rust.

## Commands

```bash
npm run tauri dev      # primary dev loop: builds Rust, starts vite on :1420, opens the window
npm run tauri build    # bundle the app
npm run dev            # frontend only (widgets will fail: no Tauri IPC available)
npm run check          # svelte-check + TypeScript (the only "lint" that exists)
cargo check            # from src-tauri/, faster than a full tauri build for Rust-only edits
```

There is no test suite and no formatter config. `npm run check` is the verification step.

## Architecture

### Rust: poll-into-snapshot, commands read from cache

`src-tauri/src/lib.rs` holds nearly all backend logic. The important structure:

- `AppSnapshot` (bottom of the file) is a single struct holding the latest value for every widget, wrapped in `Arc<RwLock<..>>` as `AppState` and registered via `.manage()`.
- `start_background_pollers()` spawns one independent async loop per data source, each with a staggered initial sleep (100ms, 450ms, 850ms, ...) so the sources don't all fire at once, then its own interval (2s system metrics, 3s Spotify, 5s Docker, 20s health, 30s Jira/Sentry).
- `#[tauri::command] get_*` functions do **not** collect anything. They clone the current snapshot field and return. Adding a data source means: struct + `collect_*()` + snapshot field + initial value in `AppState::new()` + poller loop + command + `generate_handler!` entry.
- Fields whose collection can fail are typed `Result<T, String>` inside the snapshot, so the error text crosses IPC and the widget renders it. `AppState::new()` seeds these with `Err("Loading ...")` so the UI has a defined first state.
- Blocking collectors (`sysinfo`, shelling out to `docker`/`osascript`) run under `spawn_blocking`; HTTP collectors are plain async with `reqwest`.

Data sources shell out where no crate is used: Docker via `docker ps`/`docker inspect` label parsing, Spotify via macOS `osascript` AppleScript (returns an `Err` on non-macOS). `collect_cpu_usage` deliberately refreshes twice with a 200ms sleep because `sysinfo` needs two samples.

Hardcoded, not configurable: the service-health URL list, the Sentry org/project in the API URL, and window placement (`monitors.get(1)`, i.e. second monitor, falling back to the first). Change them in `lib.rs`.

### Config and secrets

`src-tauri/src/main.rs` loads `.env` before `run()`, using `dotenvy::from_filename("../.env")` because the Rust process runs from `src-tauri/`, then `dotenv()` as a fallback. Env vars read: `JIRA_EMAIL`, `JIRA_API_TOKEN`, `JIRA_BASE_URL`, `JIRA_JQL`, `SENTRY_AUTH_TOKEN`. See `.env.example`.

### Frontend

- SPA mode: `adapter-static` with `fallback: index.html`, and `export const ssr = false` in `src/routes/+layout.ts`. Do not add server-side load functions or endpoints.
- `src/routes/+page.svelte` is the whole layout: a three-column grid, hand-assigned per column, no widget registry. Reordering widgets means editing this file.
- Every widget follows the same shape (see `RamUsage.svelte` as the reference): Svelte 5 runes (`$state`, `$props`), local `isLoading` / `loadError` state, `invoke<T>("get_...")` in `onMount` plus a `setInterval` roughly matching the Rust poll interval, cleared in `onDestroy`. TypeScript interfaces are duplicated per component to mirror the Rust structs (snake_case field names, since serde is not renaming).
- `Widget.svelte` is the shared card shell, taking `title`, optional `headerRight` snippet, and class overrides.
- External links use `openUrl` from `@tauri-apps/plugin-opener`, not `<a href>`.

### Window and styling

The window is transparent and frameless-ish: `transparent: true` + `titleBarStyle: "Overlay"` + `macOSPrivateApi: true` in `tauri.conf.json`, `background-color: transparent` on `body`, and a `data-tauri-drag-region` strip at the top of `+page.svelte` as the only drag handle. Keep `bg-transparent` on the page root.

Tailwind v4 via `@tailwindcss/vite`. The theme lives in `@theme` in `src/app.css` (`primary` aliased to teal). `tailwind.config.js` is leftover v3 scaffolding and is not read; edit `app.css` instead.

Tauri capabilities are minimal (`core:default`, `opener:default` in `src-tauri/capabilities/default.json`). Adding a plugin that the frontend calls requires a permission entry there.
