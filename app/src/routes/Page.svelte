<script>
    import ThemeSelector from "../components/ThemeSelector.svelte";
    import CodeEditor from "../components/CodeEditor.svelte";

    export let theme;

    let parseEditor;
</script>

<style type="text/scss">
    @import "../styles/globals.scss";

    h1 {
        color: var(--fg-color);
        transition: color $transition-time;
    }

    main {
        display: flex;
        flex-direction: column;
        width: 100%;
        height: 100%;
        margin: 0;
    }
</style>

<main>
    <h1>Hello world!</h1>

    <ThemeSelector bind:value="{theme}" />

    <CodeEditor bind:this="{parseEditor}" on:change="{() => {}}" mode="ebnf" />
</main>
