<script lang="ts">
  import RamUsage from "$lib/components/RamUsage.svelte";
  import CpuUsage from "$lib/components/CpuUsage.svelte";
  import DockerContainers from "$lib/components/DockerContainers.svelte";
  import SpotifyNowPlaying from "$lib/components/SpotifyNowPlaying.svelte";
  import ServiceHealth from "$lib/components/ServiceHealth.svelte";
  import SentryIssues from "$lib/components/SentryIssues.svelte";
  import JiraTickets from "$lib/components/JiraTickets.svelte";
</script>

<div class="relative h-screen overflow-hidden bg-transparent">
  <div data-tauri-drag-region class="absolute inset-x-0 top-0 z-10 h-3"></div>

  <main class="h-screen overflow-hidden">
    <div class="h-full">
      <div class="grid h-full grid-cols-1 gap-[15px] items-start md:grid-cols-2 lg:grid-cols-3">
        <div class="flex flex-col gap-[15px]">
          <RamUsage />
          <CpuUsage />
          <SpotifyNowPlaying />
        </div>

        <div class="flex h-full min-h-0 flex-col gap-[15px]">
          <ServiceHealth />
          <div class="flex-1 min-h-0">
            <SentryIssues />
          </div>
        </div>

        <div class="flex h-full min-h-0 flex-col gap-[15px]">
          <DockerContainers />
          <div class="flex-1 min-h-0">
            <JiraTickets />
          </div>
        </div>
      </div>
    </div>
  </main>
</div>
