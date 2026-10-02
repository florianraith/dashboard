<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import Widget from "./Widget.svelte";

  interface DockerContainer {
    id: string;
    name: string;
    project: string | null;
    image: string;
    status: string;
    ports: string;
    uptime: string;
    exit_status: string | null;
    running: boolean;
  }

  interface DockerStats {
    cpu_percentage: number;
    memory_used: number;
    memory_total: number;
    disk_used: number;
  }

  let containers = $state<DockerContainer[]>([]);
  let stats = $state<DockerStats | null>(null);
  let error = $state<string | null>(null);
  let isLoading = $state(true);
  let interval: number;

  let composeGroups = $derived.by(() => {
    const groups = new Map<string, DockerContainer[]>();
    for (const c of containers) {
      if (!c.project) continue;
      groups.set(c.project, [...(groups.get(c.project) ?? []), c]);
    }
    return [...groups].map(([project, items]) => ({
      project,
      items: items.sort((a, b) => a.name.localeCompare(b.name)),
    }));
  });
  let standalone = $derived(containers.filter((c) => !c.project));

  function formatGB(bytes: number): string {
    return (bytes / 1024 ** 3).toFixed(1);
  }

  // Green at 0% through yellow to red at 100%
  function usageColor(percentage: number): string {
    const hue = 140 * (1 - Math.min(Math.max(percentage, 0), 100) / 100);
    return `hsl(${hue}, 75%, 38%)`;
  }

  async function updateStats() {
    try {
      stats = await invoke<DockerStats>("get_docker_stats");
    } catch {
      stats = null;
    }
  }

  function portList(ports: string): string[] {
    return ports.split(",").map((p) => p.trim()).filter((p) => p);
  }

  async function updateContainers() {
    let keepLoading = false;
    try {
      const data = await invoke<DockerContainer[]>("get_docker_containers");
      containers = data;
      error = null;
    } catch (err) {
      console.error("Failed to get Docker containers:", err);
      const errText = String(err);
      if (errText.toLowerCase().includes("loading")) {
        keepLoading = true;
        error = null;
      } else {
        error = errText;
        containers = [];
      }
    } finally {
      isLoading = keepLoading;
    }
  }

  onMount(() => {
    updateContainers();
    updateStats();
    // Update every 5 seconds
    interval = setInterval(() => {
      updateContainers();
      updateStats();
    }, 5000);
  });

  onDestroy(() => {
    if (interval) {
      clearInterval(interval);
    }
  });
</script>

<Widget title="Docker Containers">
  {#snippet headerRight()}
    {#if stats}
      <span class="text-xs text-gray-500">
        CPU <span class="font-semibold" style:color={usageColor(stats.cpu_percentage)}>
          {stats.cpu_percentage.toFixed(1)}%
        </span>
        · RAM <span
          class="font-semibold"
          style:color={usageColor(stats.memory_total ? (stats.memory_used / stats.memory_total) * 100 : 0)}
        >
          {formatGB(stats.memory_used)} GB
        </span>
        · Disk <span class="font-semibold">{(stats.disk_used / 1e9).toFixed(1)} GB</span>
      </span>
    {/if}
  {/snippet}

  <div class="space-y-3">
    {#if isLoading}
      <p class="text-gray-500 text-sm italic">Loading containers...</p>
    {:else if error}
      <p class="text-gray-500 text-sm italic">
        {error.includes("command not found") || error.includes("Failed to execute")
          ? "Docker not installed or not running"
          : "Error loading containers"}
      </p>
    {:else if containers.length === 0}
      <p class="text-gray-500 text-sm italic">No running containers</p>
    {:else}
      <div class="space-y-3">
        {#each composeGroups as group (group.project)}
          <div>
            <h4 class="font-semibold text-gray-800 text-base truncate mb-2" title={group.project}>
              {group.project}
            </h4>
            <div class="grid grid-cols-[repeat(auto-fill,minmax(10rem,1fr))] gap-2">
              {#each group.items as container (container.id)}
                {@const ports = portList(container.ports)}
                <div
                  class="border-l-4 {container.running ? 'border-primary-500' : 'border-red-500'} px-3 py-2 bg-gray-50 rounded-r min-w-0 space-y-1"
                  title={ports.join(", ")}
                >
                  <div class="flex items-baseline justify-between gap-2">
                    <p class="text-sm font-semibold text-gray-800 truncate shrink-0 max-w-[60%]">{container.name}</p>
                    {#if container.exit_status}
                      <p class="text-xs font-medium text-red-600 whitespace-nowrap">{container.exit_status}</p>
                    {:else if ports.length}
                      <p class="text-xs font-medium text-gray-700 truncate min-w-0">{ports.join(", ")}</p>
                    {:else}
                      <p class="text-xs text-gray-400 italic whitespace-nowrap">No ports</p>
                    {/if}
                  </div>
                  <div class="flex items-baseline justify-between gap-2">
                    <p class="text-xs text-gray-600 truncate min-w-0" title={container.image}>{container.image}</p>
                    <span class="text-xs text-gray-500 whitespace-nowrap">
                      {container.running || !container.uptime ? container.uptime : `${container.uptime} ago`}
                    </span>
                  </div>
                </div>
              {/each}
            </div>
          </div>
        {/each}

        {#each standalone as container (container.id)}
          {@const ports = portList(container.ports)}
          <div class="border-l-4 border-primary-500 px-3 py-2 bg-gray-50 rounded-r">
            <!-- First row: Name - Uptime -->
            <div class="flex items-center justify-between mb-1.5">
              <h4 class="font-semibold text-gray-800 text-sm truncate mr-2" title={container.name}>
                {container.name}
              </h4>
              <span class="text-xs text-gray-500 whitespace-nowrap">
                {container.uptime}
              </span>
            </div>

            <!-- Second row: Image - Ports -->
            <div class="flex items-center justify-between">
              <p class="text-xs text-gray-600 truncate mr-2" title={container.image}>
                {container.image}
              </p>

              {#if ports.length}
                <div class="flex items-center gap-1 flex-wrap justify-end">
                  {#each ports.slice(0, 3) as port}
                    <span class="text-xs bg-primary-100 text-primary-700 px-1.5 py-0.5 rounded font-mono whitespace-nowrap">
                      {port}
                    </span>
                  {/each}
                  {#if ports.length > 3}
                    <span class="text-xs text-gray-500 whitespace-nowrap">
                      +{ports.length - 3}
                    </span>
                  {/if}
                </div>
              {:else}
                <span class="text-xs text-gray-400 italic whitespace-nowrap">No ports</span>
              {/if}
            </div>
          </div>
        {/each}
      </div>
    {/if}
  </div>
</Widget>
