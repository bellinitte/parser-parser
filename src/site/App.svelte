<script>
    import Editor from "./editor/Editor.svelte";

    export let core;

    let parseInput = "";
    let checkInput = "";
    let output = "";
    let errorLoc = null;

    function handleChange() {
        try {
            let parser = new core.EbnfParserParser(parseInput);
            output = parser.check(checkInput);
        } catch (e) {
            output = e.kind + " at position " + e.position.start;
        }
    }

    function dupsko() {
        errorLoc = { line: 1, column: 2 };
    }

    setInterval(dupsko, 10000);
</script>

<style>
    h1 {
        color: purple;
    }
</style>

<h1>Parser-parser</h1>

<Editor errorLoc="{errorLoc}" />

<!-- <textarea bind:value={parseInput} on:input={handleChange}></textarea>

<textarea bind:value={checkInput} on:input={handleChange}></textarea> -->

<p>{output}</p>
