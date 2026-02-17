<script lang="ts">
  import Widget from "$lib/components/Widget.svelte";
  import RamUsage from "$lib/components/RamUsage.svelte";
  import CpuUsage from "$lib/components/CpuUsage.svelte";
  import DockerContainers from "$lib/components/DockerContainers.svelte";
  import SpotifyNowPlaying from "$lib/components/SpotifyNowPlaying.svelte";
</script>

<div class="min-h-screen bg-gray-100">
  <div data-tauri-drag-region class="h-8"></div>

  <main class="px-8 pb-8">
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
      <RamUsage />
      
      <CpuUsage />

      <DockerContainers />

      <SpotifyNowPlaying />
    </div>
  </main>
</div>
