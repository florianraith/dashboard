<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { openUrl } from "@tauri-apps/plugin-opener";
  import Widget from "./Widget.svelte";
  import Item from "./Item.svelte";
  import Badge, { type BadgeTone } from "./Badge.svelte";
  import StatusMessage from "./StatusMessage.svelte";
  import IconButton from "./IconButton.svelte";
  import UserIcon from "~icons/tabler/user";
  import FilterIcon from "~icons/tabler/filter";
  import FilterOffIcon from "~icons/tabler/filter-off";

  interface JiraTicket {
    key: string;
    summary: string;
    status: string;
    assignee: string;
    is_mine: boolean;
    url: string;
  }

  let tickets = $state<JiraTicket[]>([]);
  let error = $state<string | null>(null);
  let isLoading = $state(true);
  let onlyMine = $state(false);
  let statusFilter = $state<string | null>(null);
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

  function getAssigneeClass(ticket: JiraTicket): string {
    return ticket.is_mine ? "text-primary-700 font-semibold" : "text-gray-600";
  }

  // Workflow order for cycling through statuses: todo, in progress, review, done, blocked, other
  const toneOrder: BadgeTone[] = ["gray", "blue", "amber", "green", "red", "primary"];

  let statuses = $derived(
    [...new Set(tickets.map((t) => t.status))].sort(
      (a, b) =>
        toneOrder.indexOf(getStatusBadgeTone(a)) - toneOrder.indexOf(getStatusBadgeTone(b)) ||
        a.localeCompare(b),
    ),
  );

  let visibleTickets = $derived(
    tickets.filter(
      (t) => (!onlyMine || t.is_mine) && (statusFilter === null || t.status === statusFilter),
    ),
  );

  // All statuses, then each status in workflow order, then back to all
  function cycleStatusFilter() {
    const next = statusFilter === null ? 0 : statuses.indexOf(statusFilter) + 1;
    statusFilter = statuses[next] ?? null;
  }

  function resetFilters() {
    onlyMine = false;
    statusFilter = null;
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
  {#snippet headerInfo()}
    <div class="flex items-center gap-1">
      <IconButton active={onlyMine} title="Only tickets assigned to me" onclick={() => (onlyMine = !onlyMine)}>
        <UserIcon class="size-4" />
      </IconButton>
      <IconButton active={statusFilter !== null} title="Cycle through ticket statuses" onclick={cycleStatusFilter}>
        <FilterIcon class="size-4" />
        {#if statusFilter}
          <span class="font-medium">{statusFilter}</span>
        {/if}
      </IconButton>
      {#if onlyMine || statusFilter !== null}
        <IconButton title="Reset filters" onclick={resetFilters}>
          <FilterOffIcon class="size-4" />
        </IconButton>
      {/if}
    </div>
  {/snippet}

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
    {:else if visibleTickets.length === 0}
      <StatusMessage>No tickets match the filters</StatusMessage>
    {:else}
      <div class="h-full min-h-0 space-y-3 overflow-y-auto pr-1">
        {#each visibleTickets as ticket (ticket.key)}
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
              <span class="text-xs {getAssigneeClass(ticket)}" title={ticket.assignee}>
                {ticket.assignee}
              </span>
            </div>
          </Item>
        {/each}
      </div>
    {/if}
  </div>
</Widget>
