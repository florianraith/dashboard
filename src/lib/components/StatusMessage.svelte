<script lang="ts">
  import type { Snippet } from "svelte";

  let { children }: { children: Snippet } = $props();
</script>

<p class="text-gray-500 text-sm italic">{@render children()}</p>
