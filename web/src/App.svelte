<script>
    import Editor from "./editor/Editor.svelte";
</script>

<style>
    :global(body) {
        background-color: rgb(40, 40, 40);
    }
</style>

{#await import('core')}
    <p>Loading module...</p>
{:then wasm}
    {#await wasm.default()}
        <p>Loading core...</p>
    {:then core}
        <Editor core="{core}" />
    {/await}
{:catch error}
    <p style="color: red">{error.message}</p>
{/await}
