<script lang="ts">
  let { title, headerRight = undefined, className = "", contentClassName = "", children } = $props();
</script>

<div class={`bg-white rounded-lg p-6 ${className}`}>
  <div class="mb-4 flex items-start justify-between gap-3">
    <h2 class="text-xl font-semibold text-primary-600">
      {title}
    </h2>
    {#if headerRight}
      <div class="shrink-0">
        {@render headerRight()}
      </div>
    {/if}
  </div>
  <div class={`text-gray-700 ${contentClassName}`}>
    {@render children()}
  </div>
</div>
