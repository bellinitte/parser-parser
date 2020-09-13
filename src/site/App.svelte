<script>
    import Editor from './editor/Editor.svelte';

    export let core;

    let parseInput = '';
    let checkInput = '';
    let output = '';

    function handleChange() {
        parser = new core.EbnfParserParser(parseInput);
        output = parser.check(checkInput);
    }
</script>

<style>
    h1 {
        color: purple;
    }
</style>

<h1>Parser-parser</h1>

<!-- <Editor /> -->

<textarea bind:value={parseInput} on:input={handleChange}></textarea>

<textarea bind:value={checkInput} on:input={handleChange}></textarea>

<p>{output}</p>
