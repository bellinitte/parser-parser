<script>
    import Editor from './editor/Editor.svelte';

    export let core;

    let parseInput = '';
    let checkInput = '';
    let output = '';

    function handleChange() {
        try {
            let parser = new core.EbnfParserParser(parseInput);
            output = parser.check(checkInput);
        } catch (e) {
            output = e.kind + ' at position ' + e.position.start;
        }
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
