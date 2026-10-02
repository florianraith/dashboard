<script lang="ts">
  import type { Snippet } from "svelte";

  let {
    active = false,
    title,
    onclick,
    children,
  }: { active?: boolean; title: string; onclick: () => void; children: Snippet } = $props();
</script>

<button
  type="button"
  class="inline-flex items-center gap-1 rounded px-1.5 py-1 text-xs transition-colors cursor-pointer {active
    ? 'bg-primary-100 text-primary-700'
    : 'text-gray-400 hover:bg-gray-100 hover:text-gray-700'}"
  aria-pressed={active}
  {title}
  {onclick}
>
  {@render children()}
</button>
