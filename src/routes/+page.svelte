<script lang="ts">
  import RamUsage from "$lib/components/RamUsage.svelte";
  import CpuUsage from "$lib/components/CpuUsage.svelte";
  import DockerContainers from "$lib/components/DockerContainers.svelte";
  import SpotifyNowPlaying from "$lib/components/SpotifyNowPlaying.svelte";
  import HealthUp from "$lib/components/HealthUp.svelte";
  import SentryIssues from "$lib/components/SentryIssues.svelte";
  import JiraTickets from "$lib/components/JiraTickets.svelte";
</script>

<div class="h-screen overflow-hidden bg-transparent">
  <div data-tauri-drag-region class="h-8"></div>

  <main class="h-[calc(100vh-2rem)] overflow-hidden px-8 pb-8">
    <div class="h-full">
      <div class="grid h-full grid-cols-1 gap-6 items-start md:grid-cols-2 lg:grid-cols-3">
        <div class="flex flex-col gap-6">
          <RamUsage />
          <CpuUsage />
          <SpotifyNowPlaying />
        </div>

        <div class="flex h-full min-h-0 flex-col gap-6">
          <HealthUp />
          <div class="flex-1 min-h-0">
            <SentryIssues />
          </div>
        </div>

        <div class="flex flex-col gap-6">
          <JiraTickets />
          <DockerContainers />
        </div>
      </div>
    </div>
  </main>
</div>
