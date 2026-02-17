<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { openUrl } from "@tauri-apps/plugin-opener";
  import Widget from "./Widget.svelte";

  interface SentryIssue {
    title: string;
    last_seen: string;
    first_seen: string;
    age: string;
    events: number;
    users: number;
    url: string;
  }

  let issues = $state<SentryIssue[]>([]);
  let error = $state<string | null>(null);
  let isLoading = $state(true);
  let interval: number;

  function formatLastSeen(value: string): string {
    const parsed = new Date(value);
    if (Number.isNaN(parsed.getTime())) {
      return value;
    }
    return parsed.toLocaleString([], {
      month: "2-digit",
      day: "2-digit",
      hour: "2-digit",
      minute: "2-digit"
    });
  }

  function formatIssueTitle(title: string): string {
    return title
      .replaceAll("/var/www/zewotherm/production", "")
      .replace(/([A-Za-z_][A-Za-z0-9_]*\\)+([A-Za-z_][A-Za-z0-9_]*)/g, "$2")
      .replace(/\s+/g, " ")
      .trim();
  }

  async function updateIssues() {
    let keepLoading = false;
    try {
      const data = await invoke<SentryIssue[]>("get_sentry_issues");
      issues = data;
      error = null;
    } catch (err) {
      console.error("Failed to get Sentry issues:", err);
      const errText = String(err);
      if (errText.toLowerCase().includes("loading")) {
        keepLoading = true;
        error = null;
      } else {
        error = errText;
        issues = [];
      }
    } finally {
      isLoading = keepLoading;
    }
  }

  async function openIssue(url: string) {
    if (!url) {
      return;
    }

    try {
      await openUrl(url);
    } catch (err) {
      console.error("Failed to open Sentry issue:", err);
    }
  }

  onMount(() => {
    updateIssues();
    interval = setInterval(updateIssues, 3000);
  });

  onDestroy(() => {
    if (interval) {
      clearInterval(interval);
    }
  });
</script>

<Widget
  title="Sentry Issues"
  className="h-full min-h-0 flex flex-col"
  contentClassName="flex-1 min-h-0"
>
  {#snippet headerRight()}
    <span class="text-xs text-gray-500"><span class="font-semibold">{issues.length}</span> issues in last 90 days</span>
  {/snippet}

  <div class="flex h-full min-h-0 flex-col gap-3">
    {#if isLoading}
      <p class="text-gray-500 text-sm italic">Loading Sentry issues...</p>
    {:else if error}
      <p class="text-gray-500 text-sm italic">
        {error.includes("SENTRY_AUTH_TOKEN")
          ? "Sentry not configured. Set SENTRY_AUTH_TOKEN in .env"
          : "Error loading issues"}
      </p>
    {:else if issues.length === 0}
      <p class="text-gray-500 text-sm italic">No issues found</p>
    {:else}
      <div class="flex-1 min-h-0 space-y-3 overflow-y-auto pr-1">
        {#each issues as issue}
          <button
            class="w-full text-left border-l-4 border-primary-500 px-3 py-2 bg-gray-50 rounded-r hover:shadow-md transition-shadow cursor-pointer"
            onclick={() => openIssue(issue.url)}
            title="Open in Sentry"
          >
            <p class="text-sm text-gray-800 font-medium leading-snug whitespace-normal break-words" title={issue.title}>
              {formatIssueTitle(issue.title)}
            </p>

            <div class="mt-2 flex items-center justify-between gap-2 text-xs text-gray-600">
              <span>Last seen: {formatLastSeen(issue.last_seen)}</span>
              <span>Age: {issue.age}</span>
            </div>

            <div class="mt-2 flex items-center gap-2">
              <span class="text-xs bg-primary-100 text-primary-700 px-1.5 py-0.5 rounded whitespace-nowrap">
                {issue.events} events
              </span>
              <span class="text-xs bg-gray-200 text-gray-700 px-1.5 py-0.5 rounded whitespace-nowrap">
                {issue.users} users
              </span>
            </div>
          </button>
        {/each}
      </div>
    {/if}
  </div>
</Widget>
