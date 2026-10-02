<script lang="ts">
  let { percentage, size = "md" }: { percentage: number; size?: "sm" | "md" } = $props();
</script>

<div class="w-full bg-gray-200 rounded-full {size === 'sm' ? 'h-2' : 'h-4'} overflow-hidden">
  <div
    class="bg-primary-500 h-full rounded-full transition-all duration-300 ease-out"
    style="width: {Math.min(percentage, 100)}%"
  ></div>
</div>
