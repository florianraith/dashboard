<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import Widget from "./Widget.svelte";

  interface SpotifyTrack {
    track_name: string;
    artist: string;
    album: string;
    artwork_url: string;
    is_playing: boolean;
  }

  let track = $state<SpotifyTrack | null>(null);
  let error = $state<string | null>(null);
  let interval: number;

  async function updateTrack() {
    try {
      const data = await invoke<SpotifyTrack>("get_spotify_track");
      track = data;
      error = null;
    } catch (err) {
      console.error("Failed to get Spotify track:", err);
      error = String(err);
      track = null;
    }
  }

  onMount(() => {
    updateTrack();
    // Update every 3 seconds
    interval = setInterval(updateTrack, 3000);
  });

  onDestroy(() => {
    if (interval) {
      clearInterval(interval);
    }
  });
</script>

{#if error || !track}
  <!-- Fallback to standard widget when no track is playing -->
  <Widget title="Spotify">
    <div class="space-y-3">
      <p class="text-gray-500 text-sm italic">
        {error?.includes("not running")
          ? "Spotify is not running"
          : error?.includes("only supported")
            ? "Only supported on macOS"
            : "No track playing"}
      </p>
    </div>
  </Widget>
{:else}
  <!-- Enhanced widget with background image -->
  <div class="relative rounded-lg overflow-hidden h-[180px] max-h-[180px]">
    <!-- Background image with blur -->
    <div
      class="absolute inset-0 bg-cover bg-center"
      style="background-image: url('{track.artwork_url}');"
    ></div>

    <!-- Dark overlay gradient -->
    <div class="absolute inset-0 bg-gradient-to-br from-black/60 via-black/50 to-black/70"></div>

    <!-- Content -->
    <div class="relative h-full flex flex-col p-6">
      <!-- Header with title (stays at top) -->
      <div class="flex items-start justify-between mb-4">
        <h2 class="text-lg font-semibold text-white/90">
          Spotify
        </h2>

        <!-- Playback indicator -->
        <div class="flex items-center gap-1.5">
          {#if track.is_playing}
            <div class="flex items-center gap-0.5">
              <div class="w-1 h-3 bg-white/90 rounded-full animate-pulse"></div>
              <div class="w-1 h-4 bg-white/90 rounded-full animate-pulse" style="animation-delay: 0.15s"></div>
              <div class="w-1 h-3 bg-white/90 rounded-full animate-pulse" style="animation-delay: 0.3s"></div>
            </div>
            <span class="text-xs text-white/90 font-medium">Playing</span>
          {:else}
            <svg class="w-3 h-3 text-white/70" fill="currentColor" viewBox="0 0 20 20">
              <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zM7 8a1 1 0 012 0v4a1 1 0 11-2 0V8zm5-1a1 1 0 00-1 1v4a1 1 0 102 0V8a1 1 0 00-1-1z" clip-rule="evenodd" />
            </svg>
            <span class="text-xs text-white/70">Paused</span>
          {/if}
        </div>
      </div>

      <!-- Spacer to push track info to bottom -->
      <div class="flex-grow"></div>

      <!-- Track info (sticks to bottom) -->
      <div class="space-y-1">
        <h3
          class="text-xl font-bold text-white truncate drop-shadow-lg"
          title={track.track_name}
          style="text-shadow: 0 2px 4px rgba(0,0,0,0.5)"
        >
          {track.track_name}
        </h3>
        <p
          class="text-sm text-white/90 truncate drop-shadow-md"
          title={track.artist}
          style="text-shadow: 0 1px 3px rgba(0,0,0,0.5)"
        >
          {track.artist}
        </p>
      </div>
    </div>
  </div>
{/if}
