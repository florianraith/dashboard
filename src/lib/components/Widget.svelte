<script lang="ts">
  let { title, children } = $props();
</script>

<div class="bg-white dark:bg-gray-800 rounded-lg shadow-md p-6">
  <h2 class="text-xl font-semibold text-primary-600 dark:text-primary-400 mb-4">
    {title}
  </h2>
  <div class="text-gray-700 dark:text-gray-300">
    {@render children()}
  </div>
</div>
