<script lang="ts">
  import "../app.css";
  import { onMount } from "svelte";

  let { children } = $props();

  onMount(() => {
    // Initialize dark mode based on system preference or localStorage
    const isDark = localStorage.getItem("darkMode") === "true" || 
                   (!localStorage.getItem("darkMode") && window.matchMedia("(prefers-color-scheme: dark)").matches);
    
    if (isDark) {
      document.documentElement.classList.add("dark");
    } else {
      document.documentElement.classList.remove("dark");
    }
  });
</script>

{@render children()}
