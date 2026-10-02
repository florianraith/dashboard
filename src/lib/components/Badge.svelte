<script lang="ts" module>
  export type BadgeTone = "primary" | "gray" | "red" | "green" | "blue" | "amber" | "emerald";

  const toneClasses: Record<BadgeTone, string> = {
    primary: "bg-primary-100 text-primary-700",
    gray: "bg-gray-200 text-gray-700",
    red: "bg-red-100 text-red-700",
    green: "bg-green-100 text-green-700",
    blue: "bg-blue-100 text-blue-700",
    amber: "bg-amber-100 text-amber-700",
    emerald: "bg-emerald-100 text-emerald-700",
  };
</script>

<script lang="ts">
  import type { Snippet } from "svelte";

  let {
    tone = "primary",
    class: className = "",
    children,
  }: { tone?: BadgeTone; class?: string; children: Snippet } = $props();
</script>

<span class="text-xs px-1.5 py-0.5 rounded whitespace-nowrap {toneClasses[tone]} {className}">
  {@render children()}
</span>
