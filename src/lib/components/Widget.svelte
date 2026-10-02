<script lang="ts">
  import type { Snippet } from "svelte";

  let {
    title,
    headerInfo = undefined,
    className = "",
    contentClassName = "",
    children,
  }: {
    title: string;
    headerInfo?: Snippet;
    className?: string;
    contentClassName?: string;
    children: Snippet;
  } = $props();
</script>

<div class={`bg-white rounded-lg p-6 ${className}`}>
  <div class="mb-4 flex items-center justify-between gap-3">
    <h2 class="text-xl font-semibold text-primary-600">
      {title}
    </h2>
    {#if headerInfo}
      <div class="shrink-0 text-xs text-gray-500">
        {@render headerInfo()}
      </div>
    {/if}
  </div>
  <div class={`text-gray-700 ${contentClassName}`}>
    {@render children()}
  </div>
</div>
