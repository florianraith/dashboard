<script lang="ts">
  import Widget from "$lib/components/Widget.svelte";
  import RamUsage from "$lib/components/RamUsage.svelte";
</script>

<main class="min-h-screen bg-gray-100 p-8">
  <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
    <RamUsage />

    <Widget title="Running Docker Containers">
      <p class="text-gray-500">Coming soon...</p>
    </Widget>

    <Widget title="Spotify">
      <p class="text-gray-500">Coming soon...</p>
    </Widget>
  </div>
</main>
