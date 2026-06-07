<script lang="ts">
    import type { Snippet } from "svelte";
    import { setContext } from "svelte";
    import { editorStore } from "$lib/stores/editor.svelte";

    setContext("tabs", editorStore);

    let { children }: { children: Snippet } = $props();
</script>

{@render children()}
