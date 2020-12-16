<script>
    import AppBar from "./AppBar.svelte";
    import Editor from "./editor/Editor.svelte";
</script>

<main>
    <AppBar id="app-bar"/>
    <div id="window">
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
    </div>
</main>

<style>
    :global(body) {
        background-color: rgb(30, 30, 30);
    }

    main {
        display: flex;
        flex-flow: column;
        height: 100%;
        vertical-align: top;
    }

    #app-bar {
        flex: 0 1 auto;
    }

    #window {
        flex: 1 1 auto;
    }
</style>
