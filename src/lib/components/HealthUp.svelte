<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { openUrl } from "@tauri-apps/plugin-opener";
  import Widget from "./Widget.svelte";

  interface ServiceHealth {
    name: string;
    url: string;
    is_up: boolean;
    status_code: number | null;
    latency_ms: number | null;
    checked_at_ms: number;
    error: string | null;
  }

  let services = $state<ServiceHealth[]>([]);
  let isLoading = $state(true);
  let error = $state<string | null>(null);
  let interval: number;

  function getLatencyClass(latencyMs: number | null): string {
    if (latencyMs === null) {
      return "text-gray-500";
    }
    if (latencyMs < 200) {
      return "text-green-600";
    }
    if (latencyMs < 500) {
      return "text-amber-600";
    }
    return "text-red-600";
  }

  function formatCheckedAt(checkedAtMs: number): string {
    if (!checkedAtMs) {
      return "n/a";
    }
    const d = new Date(checkedAtMs);
    return d.toLocaleTimeString([], {
      hour: "2-digit",
      minute: "2-digit",
      second: "2-digit"
    });
  }

  function getDisplayDomain(url: string): string {
    try {
      return new URL(url).host;
    } catch {
      return url.replace(/^https?:\/\//, "").split("/")[0] ?? url;
    }
  }

  const lastCheckedLabel = $derived(
    services.length > 0
      ? formatCheckedAt(Math.max(...services.map((service) => service.checked_at_ms)))
      : "n/a"
  );

  async function updateHealth() {
    try {
      services = await invoke<ServiceHealth[]>("get_service_health");
      error = null;
    } catch (err) {
      console.error("Failed to get service health:", err);
      error = String(err);
    } finally {
      isLoading = false;
    }
  }

  async function openService(url: string) {
    try {
      await openUrl(url);
    } catch (err) {
      console.error("Failed to open service URL:", err);
    }
  }

  onMount(() => {
    updateHealth();
    interval = setInterval(updateHealth, 3000);
  });

  onDestroy(() => {
    if (interval) {
      clearInterval(interval);
    }
  });
</script>

<Widget title="Service Health">
  {#snippet headerRight()}
    <p class="text-xs text-gray-500">Last checked: {lastCheckedLabel}</p>
  {/snippet}

  <div class="space-y-3">
    {#if isLoading}
      <p class="text-gray-500 text-sm italic">Loading health checks...</p>
    {:else if error}
      <p class="text-gray-500 text-sm italic">Unable to load health checks</p>
    {:else if services.length === 0}
      <p class="text-gray-500 text-sm italic">No services configured</p>
    {:else}
      {#each services as service}
        <button
          class="w-full text-left border-l-4 px-3 py-2 rounded-r bg-gray-50 transition-shadow hover:shadow-md {service.is_up
            ? 'border-primary-500'
            : 'border-red-500'}"
          onclick={() => openService(service.url)}
          title="Click to open service"
        >
          <div class="flex items-center justify-between mb-1.5">
            <h4 class="font-semibold text-gray-800 text-sm truncate mr-2" title={service.name}>
              {service.name}
            </h4>
            <span
              class="text-xs px-2 py-0.5 rounded whitespace-nowrap {service.is_up
                ? 'bg-primary-100 text-primary-700'
                : 'bg-red-100 text-red-700'}"
            >
              {service.is_up ? "UP" : "DOWN"}
            </span>
          </div>

          <div class="flex items-center justify-between">
            <p class="text-xs text-gray-600 truncate mr-2" title={service.url}>
              {getDisplayDomain(service.url)}
            </p>
            <span class="text-xs whitespace-nowrap {getLatencyClass(service.latency_ms)}">
              {#if service.latency_ms !== null}
                {service.latency_ms}ms
              {:else}
                n/a
              {/if}
            </span>
          </div>
        </button>
      {/each}
    {/if}
  </div>
</Widget>
