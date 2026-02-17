<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import Widget from "./Widget.svelte";

  interface CpuCore {
    core_id: number;
    usage: number;
  }

  interface CpuProcessInfo {
    name: string;
    cpu_usage: number;
  }

  interface CpuInfo {
    overall_usage: number;
    cores: CpuCore[];
    top_processes: CpuProcessInfo[];
  }

  let cpuUsage = $state<CpuInfo>({ overall_usage: 0, cores: [], top_processes: [] });
  let isLoading = $state(true);
  let loadError = $state<string | null>(null);
  let interval: number;

  async function updateCpuUsage() {
    try {
      const data = await invoke<CpuInfo>("get_cpu_usage");
      cpuUsage = data;
      loadError = null;
    } catch (error) {
      console.error("Failed to get CPU usage:", error);
      loadError = String(error);
    } finally {
      isLoading = false;
    }
  }

  onMount(() => {
    updateCpuUsage();
    // Update every 2 seconds
    interval = setInterval(updateCpuUsage, 2000);
  });

  onDestroy(() => {
    if (interval) {
      clearInterval(interval);
    }
  });
</script>

<Widget title="CPU Usage">
  <div class="space-y-4">
    {#if isLoading}
      <p class="text-gray-500 text-sm italic">Loading CPU usage...</p>
    {:else if loadError}
      <p class="text-gray-500 text-sm italic">Unable to load CPU usage</p>
    {:else}
    <!-- Overall CPU Usage -->
    <div class="flex justify-between text-sm mb-2">
      <span class="text-gray-600">Overall</span>
      <span class="font-semibold text-primary-600">
        {cpuUsage.overall_usage.toFixed(1)}%
      </span>
    </div>

    <!-- CPU Cores Grid -->
    <div class="grid grid-cols-3 gap-2">
      {#each cpuUsage.cores as core}
        <div class="space-y-1">
          <div class="flex justify-between items-center">
            <span class="text-xs text-gray-500">Core {core.core_id}</span>
            <span class="text-xs font-medium text-primary-600">{core.usage.toFixed(0)}%</span>
          </div>
          <div class="w-full bg-gray-200 rounded-full h-2 overflow-hidden">
            <div
              class="bg-primary-500 h-full rounded-full transition-all duration-300 ease-out"
              style="width: {Math.min(core.usage, 100)}%"
            ></div>
          </div>
        </div>
      {/each}
    </div>

    <!-- Top CPU Processes -->
    {#if cpuUsage.top_processes.length > 0}
      <div class="mt-4 pt-4 border-t border-gray-200">
        <h3 class="text-xs font-semibold text-gray-500 uppercase mb-2">Top Processes</h3>
        <div class="space-y-2">
          {#each cpuUsage.top_processes as process}
            <div class="flex justify-between items-center text-sm">
              <span class="text-gray-700 truncate flex-1 mr-2" title={process.name}>
                {process.name}
              </span>
              <span class="text-primary-600 font-medium text-xs min-w-[3rem] text-right">
                {process.cpu_usage.toFixed(1)}%
              </span>
            </div>
          {/each}
        </div>
      </div>
    {/if}
    {/if}
  </div>
</Widget>
