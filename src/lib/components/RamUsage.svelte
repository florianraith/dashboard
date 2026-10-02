<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import Widget from "./Widget.svelte";
  import StatusMessage from "./StatusMessage.svelte";
  import UsageBar from "./UsageBar.svelte";
  import Section from "./Section.svelte";

  interface ProcessInfo {
    name: string;
    memory: number;
    percentage: number;
  }

  interface RamInfo {
    used: number;
    total: number;
    percentage: number;
    top_processes: ProcessInfo[];
  }

  let ramUsage = $state<RamInfo>({ used: 0, total: 0, percentage: 0, top_processes: [] });
  let isLoading = $state(true);
  let loadError = $state<string | null>(null);
  let interval: number;

  function formatBytes(bytes: number): string {
    const gb = bytes / (1024 * 1024 * 1024);
    return gb.toFixed(2);
  }

  function formatMB(bytes: number): string {
    const mb = bytes / (1024 * 1024);
    return mb.toFixed(0);
  }

  async function updateRamUsage() {
    try {
      const data = await invoke<RamInfo>("get_ram_usage");
      ramUsage = data;
      loadError = null;
    } catch (error) {
      console.error("Failed to get RAM usage:", error);
      loadError = String(error);
    } finally {
      isLoading = false;
    }
  }

  onMount(() => {
    updateRamUsage();
    // Update every 2 seconds
    interval = setInterval(updateRamUsage, 2000);
  });

  onDestroy(() => {
    if (interval) {
      clearInterval(interval);
    }
  });
</script>

<Widget title="RAM Usage">
  <div class="space-y-4">
    {#if isLoading}
      <StatusMessage>Loading RAM usage...</StatusMessage>
    {:else if loadError}
      <StatusMessage>Unable to load RAM usage</StatusMessage>
    {:else}
    <div class="flex justify-between text-sm">
      <span class="text-gray-600">
        {formatBytes(ramUsage.used)} GB / {formatBytes(ramUsage.total)} GB
      </span>
      <span class="font-semibold text-primary-600">
        {ramUsage.percentage.toFixed(1)}%
      </span>
    </div>
    
    <UsageBar percentage={ramUsage.percentage} />

    {#if ramUsage.top_processes.length > 0}
      <Section label="Top Processes">
        <div class="space-y-2">
          {#each ramUsage.top_processes as process}
            <div class="flex justify-between items-center text-sm">
              <span class="text-gray-700 truncate flex-1 mr-2" title={process.name}>
                {process.name}
              </span>
              <div class="flex items-center gap-2">
                <span class="text-gray-500 text-xs">
                  {formatMB(process.memory)} MB
                </span>
                <span class="text-primary-600 font-medium text-xs min-w-[3rem] text-right">
                  {process.percentage.toFixed(1)}%
                </span>
              </div>
            </div>
          {/each}
        </div>
      </Section>
    {/if}
    {/if}
  </div>
</Widget>
