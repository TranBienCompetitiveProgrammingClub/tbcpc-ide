<script lang="ts">
    import { getContext } from "svelte";
    import Tab from "./Tab.svelte";
    import { open, save } from "@tauri-apps/plugin-dialog";
    import { readTextFile, writeTextFile } from "@tauri-apps/plugin-fs";

    const ctx = getContext<any>("tabs");
    let tabs = $derived(ctx.tabs);
    let activeId = $derived(ctx.activeId);
    let activeTab = $derived(tabs.find((t: any) => t.id === activeId));
    let showModal = $state(false);
    let newFileName = $state("");
    let inputEl: HTMLInputElement;

    $effect(() => {
        if (showModal && inputEl) {
            inputEl.focus();
        }
    });

    async function openFile() {
        const path = await open({
            multiple: false,
            filters: [
                { name: "Code", extensions: ["cpp", "cc", "c", "py", "txt"] },
            ],
        });
        if (!path) return;
        const content = await readTextFile(path as string);
        const label = (path as string).split("/").pop() ?? "untitled";
        ctx.addTab(label, content, path as string);
    }

    async function saveFile() {
        if (!activeTab) return;
        let filePath = activeTab.filePath;
        if (!filePath) {
            filePath = await save({
                defaultPath: activeTab.label,
                filters: [
                    {
                        name: "Code",
                        extensions: ["cpp", "cc", "c", "py", "txt"],
                    },
                ],
            });
            if (!filePath) return;
            ctx.setFilePath(activeTab.id, filePath as string);
        }
        await writeTextFile(filePath as string, activeTab.content);
    }

    function confirm() {
        if (newFileName.trim()) {
            ctx.addTab(newFileName.trim());
            newFileName = "";
            showModal = false;
        }
    }
</script>

{#if showModal}
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div
        class="fixed inset-0 bg-black/50 flex items-center justify-center z-50"
        onclick={() => (showModal = false)}
        onkeydown={(e) => e.key === "Escape" && (showModal = false)}
    >
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <div
            class="bg-base-200 border border-base-300 rounded-lg p-4 flex flex-col gap-3 w-64"
            onclick={(e) => e.stopPropagation()}
            onkeydown={(e) => e.stopPropagation()}
        >
            <p class="text-sm text-base-content">File name:</p>
            <input
                class="input input-sm bg-base-300 border-base-300 text-base-content w-full font-mono"
                type="text"
                placeholder="untitled.cpp"
                bind:value={newFileName}
                bind:this={inputEl}
                onkeydown={(e) => e.key === "Enter" && confirm()}
            />
            <div class="flex justify-end gap-2">
                <button
                    class="btn btn-sm btn-ghost"
                    onclick={() => (showModal = false)}>Cancel</button
                >
                <button class="btn btn-sm btn-primary" onclick={confirm}
                    >OK</button
                >
            </div>
        </div>
    </div>
{/if}

<div
    class="flex flex-row items-center h-full flex-1 overflow-x-auto overflow-y-hidden"
>
    <!-- Open file button -->
    <button
        class="px-3 h-full text-base-content/50 hover:text-base-content shrink-0 tooltip tooltip-bottom"
        data-tip="Open file"
        onclick={openFile}
    >
        <svg
            xmlns="http://www.w3.org/2000/svg"
            class="h-4 w-4"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
        >
            <path
                d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"
            ></path>
        </svg>
    </button>

    <!-- Save file button -->
    <button
        class="px-3 h-full text-base-content/50 hover:text-base-content shrink-0 tooltip tooltip-bottom"
        data-tip={activeTab?.filePath ? "Save file" : "Save as"}
        onclick={saveFile}
        disabled={!activeTab}
    >
        <svg
            xmlns="http://www.w3.org/2000/svg"
            class="h-4 w-4"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
        >
            <path
                d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"
            ></path>
            <polyline points="17 21 17 13 7 13 7 21"></polyline>
            <polyline points="7 3 7 8 15 8"></polyline>
        </svg>
    </button>

    {#each tabs as tab (tab.id)}
        <Tab
            label={tab.label}
            active={tab.id === activeId}
            onclick={() => ctx.setActive(tab.id)}
            onclose={() => ctx.removeTab(tab.id)}
        />
    {/each}

    <button
        class="px-3 h-full text-lg text-base-content/50 hover:text-base-content shrink-0"
        onclick={() => (showModal = true)}>+</button
    >
</div>
