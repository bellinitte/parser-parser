<script>
    import Editor from "./editor/Editor.svelte";
</script>

<style>
    h1 {
        color: purple;
    }
</style>

<h1>Parser-parser</h1>

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
