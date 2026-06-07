<script lang="ts">
    import { getContext } from "svelte";
    import { PaneGroup, Pane, PaneResizer } from "paneforge";
    import TextBuffer from "$lib/components/TextBuffer.svelte";

    const ctx = getContext<any>("tabs");
    let tabs = $derived(ctx.tabs);
    let activeId = $derived(ctx.activeId);
    let activeTab = $derived(tabs.find((t: any) => t.id === activeId));
    let inputBuffer = $derived(ctx.inputBuffer);
    let outputBuffer = $derived(ctx.outputBuffer);

    let baseName = $derived(
        activeTab?.label.replace(/\.[^.]+$/, "") ?? "untitled",
    );
    let inputName = $derived(`${baseName}.inp`);
    let outputName = $derived(`${baseName}.out`);
</script>

<PaneGroup direction="vertical">
    <Pane defaultSize={50} minSize={20}>
        <TextBuffer
            filename={inputName}
            content={inputBuffer}
            onchange={(v) => ctx.setInputBuffer(v)}
        />
    </Pane>
    <PaneResizer
        class="h-1 bg-base-300 hover:bg-primary transition-colors cursor-row-resize"
    />
    <Pane defaultSize={50} minSize={20}>
        <TextBuffer
            filename={outputName}
            content={outputBuffer}
            onchange={(v) => ctx.setOutputBuffer(v)}
        />
    </Pane>
</PaneGroup>
