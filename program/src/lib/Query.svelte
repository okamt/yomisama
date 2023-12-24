<script lang="ts">
    import type { ComponentProps } from "svelte";
    import QueryEntry from "./QueryEntry.svelte";

    export const entries: ComponentProps<QueryEntry>[] = [];
</script>

<main class="flex flex-col space-y-8 h-screen">
    {#each entries as entry}
        <QueryEntry {...entry} />
    {/each}
</main>
