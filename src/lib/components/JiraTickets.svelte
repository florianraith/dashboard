<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { openUrl } from "@tauri-apps/plugin-opener";
  import Widget from "./Widget.svelte";
  import Item from "./Item.svelte";
  import Badge, { type BadgeTone } from "./Badge.svelte";
  import StatusMessage from "./StatusMessage.svelte";

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

  function getStatusBadgeTone(status: string): BadgeTone {
    const statusLower = status.toLowerCase();
    if (statusLower.includes("done") || statusLower.includes("closed")) {
      return "green";
    }
    if (statusLower.includes("progress") || statusLower.includes("development")) {
      return "blue";
    }
    if (statusLower.includes("review") || statusLower.includes("test") || statusLower.includes("qa")) {
      return "amber";
    }
    if (
      statusLower.includes("todo") ||
      statusLower.includes("backlog") ||
      statusLower.includes("open") ||
      statusLower.includes("zu erledigen")
    ) {
      return "gray";
    }
    if (statusLower.includes("blocked")) {
      return "red";
    }
    return "primary";
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

<Widget
  title="Jira Tickets"
  className="h-full flex flex-col"
  contentClassName="flex-1 min-h-0"
>
  <div class="h-full min-h-0">
    {#if isLoading}
      <StatusMessage>Loading Jira tickets...</StatusMessage>
    {:else if error}
      <StatusMessage>
        {error.includes("environment variable") || error.includes("JIRA_")
          ? "Jira not configured. Set JIRA_EMAIL and JIRA_API_TOKEN in .env"
          : error.includes("401") || error.includes("403")
            ? "Authentication failed. Check your email and API token in .env"
            : "Error loading tickets"}
      </StatusMessage>
    {:else if tickets.length === 0}
      <StatusMessage>No tickets found</StatusMessage>
    {:else}
      <div class="h-full min-h-0 space-y-3 overflow-y-auto pr-1">
        {#each tickets as ticket}
          <Item onclick={() => openTicket(ticket.url)} title="Click to open in browser">
            <!-- First row: Title -->
            <p class="text-sm text-gray-800 font-medium leading-snug whitespace-normal break-words" title={ticket.summary}>
              {ticket.summary}
            </p>

            <!-- Second row: Key - Status -->
            <div class="flex items-center justify-between mt-2 gap-2">
              <span class="text-xs text-gray-500 whitespace-nowrap">
                {ticket.key}
              </span>
              <Badge tone={getStatusBadgeTone(ticket.status)}>
                {ticket.status}
              </Badge>
            </div>

            <!-- Third row: Assignee (plain text) -->
            <div class="mt-1">
              <span class="text-xs {getAssigneeClass(ticket.assignee)}" title={ticket.assignee}>
                {ticket.assignee}
              </span>
            </div>
          </Item>
        {/each}
      </div>
    {/if}
  </div>
</Widget>
