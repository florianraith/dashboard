<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import Widget from "./Widget.svelte";

  interface DockerContainer {
    id: string;
    name: string;
    image: string;
    status: string;
    ports: string;
    uptime: string;
  }

  let containers = $state<DockerContainer[]>([]);
  let error = $state<string | null>(null);
  let interval: number;

  async function updateContainers() {
    try {
      const data = await invoke<DockerContainer[]>("get_docker_containers");
      containers = data;
      error = null;
    } catch (err) {
      console.error("Failed to get Docker containers:", err);
      error = String(err);
      containers = [];
    }
  }

  onMount(() => {
    updateContainers();
    // Update every 5 seconds
    interval = setInterval(updateContainers, 5000);
  });

  onDestroy(() => {
    if (interval) {
      clearInterval(interval);
    }
  });
</script>

<Widget title="Running Docker Containers">
  <div class="space-y-3">
    {#if error}
      <p class="text-gray-500 text-sm italic">
        {error.includes("command not found") || error.includes("Failed to execute")
          ? "Docker not installed or not running"
          : "Error loading containers"}
      </p>
    {:else if containers.length === 0}
      <p class="text-gray-500 text-sm italic">No running containers</p>
    {:else}
      <div class="space-y-3">
        {#each containers as container}
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

              {#if container.ports}
                <div class="flex items-center gap-1 flex-wrap justify-end">
                  {#each container.ports.split(',').map(p => p.trim()).filter(p => p).slice(0, 3) as port}
                    <span class="text-xs bg-primary-100 text-primary-700 px-1.5 py-0.5 rounded font-mono whitespace-nowrap">
                      {port}
                    </span>
                  {/each}
                  {#if container.ports.split(',').filter(p => p.trim()).length > 3}
                    <span class="text-xs text-gray-500 whitespace-nowrap">
                      +{container.ports.split(',').filter(p => p.trim()).length - 3}
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
