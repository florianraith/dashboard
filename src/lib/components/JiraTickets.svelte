<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { openUrl } from "@tauri-apps/plugin-opener";
  import Widget from "./Widget.svelte";

  interface JiraTicket {
    key: string;
    summary: string;
    status: string;
    assignee: string;
    url: string;
  }

  let tickets = $state<JiraTicket[]>([]);
  let error = $state<string | null>(null);
  let interval: number;

  function getStatusColor(status: string): string {
    const statusLower = status.toLowerCase();
    if (statusLower.includes("done") || statusLower.includes("closed")) {
      return "border-green-500 bg-green-50";
    } else if (statusLower.includes("blocked")) {
      return "border-red-500 bg-red-50";
    } else {
      return "border-primary-500 bg-primary-50";
    }
  }

  function getStatusBadgeColor(status: string): string {
    const statusLower = status.toLowerCase();
    if (statusLower.includes("done") || statusLower.includes("closed")) {
      return "bg-green-100 text-green-700";
    } else if (statusLower.includes("blocked")) {
      return "bg-red-100 text-red-700";
    } else {
      return "bg-primary-100 text-primary-700";
    }
  }

  async function updateTickets() {
    try {
      const data = await invoke<JiraTicket[]>("get_jira_tickets");
      tickets = data;
      error = null;
    } catch (err) {
      console.error("Failed to get Jira tickets:", err);
      error = String(err);
      tickets = [];
    }
  }

  async function openTicket(url: string) {
    try {
      await openUrl(url);
    } catch (err) {
      console.error("Failed to open ticket:", err);
    }
  }

  onMount(() => {
    updateTickets();
    // Update every 30 seconds
    interval = setInterval(updateTickets, 30000);
  });

  onDestroy(() => {
    if (interval) {
      clearInterval(interval);
    }
  });
</script>

<Widget title="Jira Tickets">
  <div class="space-y-3">
    {#if error}
      <p class="text-gray-500 text-sm italic">
        {error.includes("environment variable") || error.includes("JIRA_")
          ? "Jira not configured. Set JIRA_EMAIL and JIRA_API_TOKEN in .env"
          : error.includes("401") || error.includes("403")
            ? "Authentication failed. Check your email and API token in .env"
            : "Error loading tickets"}
      </p>
    {:else if tickets.length === 0}
      <p class="text-gray-500 text-sm italic">No tickets found</p>
    {:else}
      <div class="space-y-3">
        {#each tickets as ticket}
          <button
            class="w-full text-left border-l-4 {getStatusColor(ticket.status)} px-3 py-2 rounded-r hover:shadow-md hover:ring-1 hover:ring-primary-300 transition-all cursor-pointer"
            onclick={() => openTicket(ticket.url)}
            title="Click to open in browser"
          >
            <!-- First row: Key - Status Badge -->
            <div class="flex items-center justify-between mb-1.5">
              <h4 class="font-semibold text-primary-800 text-sm" title={ticket.key}>
                {ticket.key}
              </h4>
              <span class="text-xs px-2 py-0.5 rounded {getStatusBadgeColor(ticket.status)} whitespace-nowrap">
                {ticket.status}
              </span>
            </div>

            <!-- Second row: Summary -->
            <p class="text-sm text-gray-800 mb-1.5 line-clamp-2" title={ticket.summary}>
              {ticket.summary}
            </p>

            <!-- Third row: Assignee -->
            <div class="flex items-center gap-1.5">
              <span class="text-xs text-primary-700">👤</span>
              <span class="text-xs text-gray-700">{ticket.assignee}</span>
            </div>
          </button>
        {/each}
      </div>
    {/if}
  </div>
</Widget>

<style>
  .line-clamp-2 {
    display: -webkit-box;
    -webkit-line-clamp: 2;
    -webkit-box-orient: vertical;
    overflow: hidden;
  }
</style>
