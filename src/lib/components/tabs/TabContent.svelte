<script lang="ts">
    import { getContext } from "svelte";
    import TextBuffer from "$lib/components/TextBuffer.svelte";

    const ctx = getContext<any>("tabs");
    let tabs = $derived(ctx.tabs);
    let activeId = $derived(ctx.activeId);
    let activeTab = $derived(tabs.find((t: any) => t.id === activeId));
</script>

<div class="w-full h-full overflow-hidden">
    {#if activeTab}
        <TextBuffer
            filename={activeTab.label}
            content={activeTab.content}
            onchange={(val) => (activeTab.content = val)}
        />
    {:else}
        <div class="w-full h-full flex items-center justify-center">
            <p class="text-base-content/30 text-sm font-mono">No file open</p>
        </div>
    {/if}
</div>
