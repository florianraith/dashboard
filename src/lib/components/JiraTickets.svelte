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
  let isLoading = $state(true);
  let interval: number;

  function getStatusBadgeColor(status: string): string {
    const statusLower = status.toLowerCase();
    if (statusLower.includes("done") || statusLower.includes("closed")) {
      return "bg-green-100 text-green-700";
    }
    if (statusLower.includes("progress") || statusLower.includes("development")) {
      return "bg-blue-100 text-blue-700";
    }
    if (statusLower.includes("review") || statusLower.includes("test") || statusLower.includes("qa")) {
      return "bg-amber-100 text-amber-700";
    }
    if (
      statusLower.includes("todo") ||
      statusLower.includes("backlog") ||
      statusLower.includes("open") ||
      statusLower.includes("zu erledigen")
    ) {
      return "bg-gray-100 text-gray-700";
    }
    if (statusLower.includes("blocked")) {
      return "bg-red-100 text-red-700";
    }
    return "bg-primary-100 text-primary-700";
  }

  function getAssigneeClass(assignee: string): string {
    if (assignee.trim().toLowerCase() === "florian raith") {
      return "text-primary-700 font-semibold";
    }
    return "text-gray-600";
  }

  async function updateTickets() {
    let keepLoading = false;
    try {
      const data = await invoke<JiraTicket[]>("get_jira_tickets");
      tickets = data;
      error = null;
    } catch (err) {
      console.error("Failed to get Jira tickets:", err);
      const errText = String(err);
      if (errText.toLowerCase().includes("loading")) {
        keepLoading = true;
        error = null;
      } else {
        error = errText;
        tickets = [];
      }
    } finally {
      isLoading = keepLoading;
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
  <div class="space-y-3 max-h-[420px]">
    {#if isLoading}
      <p class="text-gray-500 text-sm italic">Loading Jira tickets...</p>
    {:else if error}
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
      <div class="space-y-3 max-h-[320px] overflow-y-auto pr-1">
        {#each tickets as ticket}
          <button
            class="w-full text-left border-l-4 border-primary-500 px-3 py-2 bg-gray-50 rounded-r hover:shadow-md transition-shadow cursor-pointer"
            onclick={() => openTicket(ticket.url)}
            title="Click to open in browser"
          >
            <!-- First row: Title -->
            <p class="text-sm text-gray-800 font-medium leading-snug whitespace-normal break-words" title={ticket.summary}>
              {ticket.summary}
            </p>

            <!-- Second row: Key - Status -->
            <div class="flex items-center justify-between mt-2 gap-2">
              <span class="text-xs text-gray-500 whitespace-nowrap">
                {ticket.key}
              </span>
              <span class="text-xs px-2 py-0.5 rounded {getStatusBadgeColor(ticket.status)} whitespace-nowrap">
                {ticket.status}
              </span>
            </div>

            <!-- Third row: Assignee (plain text) -->
            <div class="mt-1">
              <span class="text-xs {getAssigneeClass(ticket.assignee)}" title={ticket.assignee}>
                {ticket.assignee}
              </span>
            </div>
          </button>
        {/each}
      </div>
    {/if}
  </div>
</Widget>
