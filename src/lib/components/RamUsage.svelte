<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import Widget from "./Widget.svelte";

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
      <p class="text-gray-500 text-sm italic">Loading RAM usage...</p>
    {:else if loadError}
      <p class="text-gray-500 text-sm italic">Unable to load RAM usage</p>
    {:else}
    <div class="flex justify-between text-sm">
      <span class="text-gray-600">
        {formatBytes(ramUsage.used)} GB / {formatBytes(ramUsage.total)} GB
      </span>
      <span class="font-semibold text-primary-600">
        {ramUsage.percentage.toFixed(1)}%
      </span>
    </div>
    
    <div class="w-full bg-gray-200 rounded-full h-4 overflow-hidden">
      <div
        class="bg-primary-500 h-full rounded-full transition-all duration-300 ease-out"
        style="width: {ramUsage.percentage}%"
      ></div>
    </div>

    {#if ramUsage.top_processes.length > 0}
      <div class="mt-4 pt-4 border-t border-gray-200">
        <h3 class="text-xs font-semibold text-gray-500 uppercase mb-2">Top Processes</h3>
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
      </div>
    {/if}
    {/if}
  </div>
</Widget>
