<script lang="ts">
    import type { Snippet } from "svelte";
    import { onMount } from "svelte";
    import { editorStore } from "$lib/stores/editor.svelte";
    import "../app.css";

    let { children }: { children: Snippet } = $props();

    onMount(() => {
        document.documentElement.setAttribute(
            "data-theme",
            editorStore.uiTheme,
        );
    });
</script>

<div class="w-full h-full overflow-hidden">
    {@render children()}
</div>
