<script lang="ts">
    import { onMount, onDestroy, getContext } from "svelte";
    import { EditorView, basicSetup } from "codemirror";
    import { EditorState, Compartment } from "@codemirror/state";
    import { cpp } from "@codemirror/lang-cpp";
    import { python } from "@codemirror/lang-python";
    import type { Extension } from "@codemirror/state";

    let {
        filename = "untitled.cpp",
        content = "",
        onchange,
    }: {
        filename?: string;
        content?: string;
        onchange?: (value: string) => void;
    } = $props();

    const ctx = getContext<any>("tabs");
    let fontSize = $derived(ctx.fontSize);
    let editorTheme = $derived(ctx.editorTheme);
    let container: HTMLDivElement;
    let view: EditorView;
    let previousId = $state<string | null>(null);

    const fontSizeCompartment = new Compartment();
    const themeCompartment = new Compartment();

    function getLanguage(name: string): Extension {
        if (
            name.endsWith(".cpp") ||
            name.endsWith(".cc") ||
            name.endsWith(".c")
        )
            return cpp();
        if (name.endsWith(".py")) return python();
        return [];
    }

    function fontSizeTheme(size: number) {
        return EditorView.theme({
            "&": { height: "100%" },
            ".cm-scroller": { overflow: "auto", fontSize: `${size}px` },
        });
    }

    function createState(doc: string, name: string) {
        return EditorState.create({
            doc,
            extensions: [
                basicSetup,
                themeCompartment.of(editorTheme),
                getLanguage(name),
                fontSizeCompartment.of(fontSizeTheme(fontSize)),
                EditorView.updateListener.of((update) => {
                    if (update.docChanged) {
                        onchange?.(update.state.doc.toString());
                    }
                }),
            ],
        });
    }

    onMount(() => {
        previousId = ctx.activeId;
        view = new EditorView({
            state: createState(content, filename),
            parent: container,
        });
    });

    // swap state when active tab changes
    $effect(() => {
        const activeId = ctx.activeId;
        if (view && activeId !== previousId) {
            previousId = activeId;
            view.setState(createState(content, filename));
        }
    });

    // font size
    $effect(() => {
        if (view) {
            view.dispatch({
                effects: fontSizeCompartment.reconfigure(
                    fontSizeTheme(fontSize),
                ),
            });
        }
    });

    // editor theme
    $effect(() => {
        if (view) {
            view.dispatch({
                effects: themeCompartment.reconfigure(editorTheme),
            });
        }
    });

    // sync external content changes
    $effect(() => {
        const newContent = content;
        if (view && newContent !== view.state.doc.toString()) {
            view.dispatch({
                changes: {
                    from: 0,
                    to: view.state.doc.length,
                    insert: newContent,
                },
            });
        }
    });

    onDestroy(() => view?.destroy());
</script>

<div class="w-full h-full overflow-hidden" bind:this={container}></div>

<style>
    div :global(.cm-editor) {
        height: 100%;
    }

    div :global(.cm-scroller) {
        font-family: "JetBrains Mono", monospace;
    }
</style>
