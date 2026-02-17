<script lang="ts">
  let { title, children } = $props();
</script>

<div class="bg-white rounded-lg shadow p-6">
  <h2 class="text-xl font-semibold text-primary-600 mb-4">
    {title}
  </h2>
  <div class="text-gray-700">
    {@render children()}
  </div>
</div>
