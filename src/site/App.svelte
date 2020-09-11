<script>
    import Editor from './editor/Editor.svelte';

    export let core;

    let input = '<span number>128</span>';
    let output = '';

    function handleChange() {
        output = core.parse(input);
    }
</script>

<style>
    h1 {
        color: purple;
    }
</style>

<h1>Parser-parser</h1>

<Editor />

<!-- <textarea bind:value={input} on:input={handleChange}></textarea> -->

<!-- <p>{output}</p> -->
