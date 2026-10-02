<script lang="ts">
  import type { Snippet } from "svelte";

  let { label, children }: { label: string; children: Snippet } = $props();
</script>

<div class="mt-4 pt-4 border-t border-gray-200">
  <h3 class="text-xs font-semibold text-gray-500 uppercase mb-2">{label}</h3>
  {@render children()}
</div>
