<script lang="ts">
    import { PaneGroup, Pane, PaneResizer } from "paneforge";
    import TabGroup from "$lib/components/tabs/TabGroup.svelte";
    import TabBar from "$lib/components/tabs/TabBar.svelte";
    import TabContent from "$lib/components/tabs/TabContent.svelte";
    import ToolBar from "$lib/components/ToolBar.svelte";
    import IOPanes from "$lib/components/IOPanes.svelte";
    import AppShell from "$lib/components/AppShell.svelte";
</script>

<div class="flex flex-col w-full h-screen">
    <TabGroup>
        <!-- Toolbar row -->
        <AppShell>
            <div
                class="flex flex-row items-center h-10 bg-base-200 border-b border-base-300 shrink-0"
            >
                <TabBar />
                <ToolBar />
            </div>

            <!-- Body -->
            <div class="flex flex-1 min-h-0">
                <PaneGroup direction="horizontal">
                    <Pane defaultSize={60} minSize={20}>
                        <TabContent />
                    </Pane>
                    <PaneResizer
                        class="w-1 bg-base-300 hover:bg-primary transition-colors cursor-col-resize"
                    />
                    <Pane defaultSize={40} minSize={20}>
                        <IOPanes />
                    </Pane>
                </PaneGroup>
            </div>
        </AppShell>
    </TabGroup>
</div>
