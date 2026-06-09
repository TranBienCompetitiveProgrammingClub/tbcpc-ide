<script lang="ts">
    import {
        getContext,
        setContext,
        onMount,
        onDestroy,
        type Snippet,
    } from "svelte";
    import { registerWindowKeybinds, getEditorKeybinds } from "$lib/keybinds";
    import { open, save } from "@tauri-apps/plugin-dialog";
    import { readTextFile, writeTextFile } from "@tauri-apps/plugin-fs";
    import { invoke } from "@tauri-apps/api/core";

    let { children }: { children: Snippet } = $props();

    const ctx = getContext<any>("tabs");
    let tabs = $derived(ctx.tabs);
    let activeId = $derived(ctx.activeId);
    let activeTab = $derived(tabs.find((t: any) => t.id === activeId));
    let inputBuffer = $derived(ctx.inputBuffer);

    // expose showNewFileModal to TabBar via context
    let showNewFileModal = $state<(() => void) | null>(null);

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

    function getLanguage(filename: string): string {
        if (filename.endsWith(".py")) return "python";
        return "cpp";
    }

    async function runCode() {
        if (!activeTab) return;
        try {
            const result = await invoke<string>("run_code", {
                code: activeTab.content,
                input: inputBuffer,
                language: getLanguage(activeTab.label),
                ioMode: activeTab.ioMode,
                filePath: activeTab.filePath ?? activeTab.label,
            });
            ctx.setOutputBuffer(result);
        } catch (e) {
            ctx.setOutputBuffer(String(e));
        }
    }

    // expose actions via context so TabBar and ToolBar can call them
    setContext("actions", {
        openFile,
        saveFile,
        runCode,
        get editorKeybinds() {
            return getEditorKeybinds({
                openFile,
                saveFile,
                showNewFileModal: () => showNewFileModal?.(),
                runCode,
            });
        },
        get showNewFileModal() {
            return showNewFileModal;
        },
        setShowNewFileModal(fn: () => void) {
            showNewFileModal = fn;
        },
    });

    let unregister: () => void;

    onMount(() => {
        unregister = registerWindowKeybinds({
            openFile,
            saveFile,
            showNewFileModal: () => showNewFileModal?.(),
            runCode,
        });
    });

    onDestroy(() => unregister?.());
</script>

{@render children()}
