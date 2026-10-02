<script lang="ts">
  import type { Snippet } from "svelte";

  let {
    alert = false,
    onclick = undefined,
    title = undefined,
    class: className = "",
    children,
  }: {
    alert?: boolean;
    onclick?: () => void;
    title?: string;
    class?: string;
    children: Snippet;
  } = $props();

  let classes = $derived(
    `border-l-4 ${alert ? "border-red-500" : "border-primary-500"} px-3 py-2 bg-gray-50 rounded-r ${className}`,
  );
</script>

{#if onclick}
  <button class="w-full text-left hover:shadow-md transition-shadow cursor-pointer {classes}" {onclick} {title}>
    {@render children()}
  </button>
{:else}
  <div class={classes} {title}>
    {@render children()}
  </div>
{/if}
