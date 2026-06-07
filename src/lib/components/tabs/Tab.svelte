<script lang="ts">
    let {
        label,
        active,
        onclick,
        onclose,
    }: {
        label: string;
        active: boolean;
        onclick: () => void;
        onclose: () => void;
    } = $props();
</script>

<div
    class={`flex flex-row items-center gap-2 px-3 h-full cursor-pointer select-none border-r border-base-300 text-sm shrink-0 ${active ? "bg-base-100 border-t-2 border-t-primary text-base-content" : "bg-base-200 text-base-content/50 hover:bg-base-100 hover:text-base-content"}`}
    role="tab"
    tabindex="0"
    {onclick}
    onkeydown={(e) => e.key === "Enter" && onclick()}
>
    <span class="max-w-24 truncate">{label}</span>
    <button
        class="hover:text-base-content text-base-content/30 text-base leading-none rounded px-1"
        onclick={(e) => {
            e.stopPropagation();
            onclose();
        }}>×</button
    >
</div>
