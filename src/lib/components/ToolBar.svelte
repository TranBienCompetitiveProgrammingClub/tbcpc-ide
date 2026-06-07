<script lang="ts">
    import { getContext } from "svelte";
    import { invoke } from "@tauri-apps/api/core";

    const ctx = getContext<any>("tabs");
    let fontSize = $derived(ctx.fontSize);
    let isDark = $derived(ctx.isDark);
    let tabs = $derived(ctx.tabs);
    let activeId = $derived(ctx.activeId);
    let activeTab = $derived(tabs.find((t: any) => t.id === activeId));
    let inputBuffer = $derived(ctx.inputBuffer);

    let running = $state(false);

    function getLanguage(filename: string): string {
        if (filename.endsWith(".py")) return "python";
        return "cpp";
    }

    async function run() {
        if (!activeTab || running) return;
        running = true;
        try {
            const result = await invoke<string>("run_code", {
                code: activeTab.content,
                input: inputBuffer,
                language: getLanguage(activeTab.label),
            });
            ctx.setOutputBuffer(result);
        } catch (e) {
            ctx.setOutputBuffer(String(e));
        } finally {
            running = false;
        }
    }
</script>

<div
    class="flex flex-row items-center gap-3 px-3 h-full shrink-0 border-l border-base-300"
>
    <!-- Font size -->
    <div class="flex flex-row items-center gap-1">
        <button
            class="btn btn-xs btn-ghost font-mono"
            onclick={() => ctx.setFontSize(Math.max(8, fontSize - 1))}>−</button
        >
        <span class="text-xs font-mono text-base-content min-w-10 text-center"
            >{fontSize}px</span
        >
        <button
            class="btn btn-xs btn-ghost font-mono"
            onclick={() => ctx.setFontSize(Math.min(32, fontSize + 1))}
            >+</button
        >
    </div>

    <!-- IO mode toggle -->
    {#if activeTab}
        <div class="flex flex-row items-center gap-2 text-xs font-mono">
            <span
                class={activeTab.ioMode === "stdio"
                    ? "text-base-content"
                    : "text-base-content/40"}>Stdio</span
            >
            <input
                type="checkbox"
                class="toggle toggle-xs toggle-primary"
                checked={activeTab.ioMode === "file"}
                onchange={() =>
                    (activeTab.ioMode =
                        activeTab.ioMode === "stdio" ? "file" : "stdio")}
            />
            <span
                class={activeTab.ioMode === "file"
                    ? "text-base-content"
                    : "text-base-content/40"}>File</span
            >
        </div>
    {/if}

    <!-- Theme toggle -->
    <button class="btn btn-xs btn-ghost" onclick={() => ctx.toggleTheme()}>
        {isDark ? "☀" : "☾"}
    </button>

    <!-- Run -->
    <button
        class="btn btn-xs btn-success font-mono font-bold"
        onclick={run}
        disabled={running || !activeTab}
    >
        {running ? "..." : "▶ Run"}
    </button>
</div>
