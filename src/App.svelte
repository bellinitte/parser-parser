<script>
    export let core;

    let input = '';
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

<textarea bind:value={input} on:input={handleChange}></textarea>

<p>{output}</p>
