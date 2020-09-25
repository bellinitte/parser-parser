<script>
    import CodeMirror from "./CodeMirror.svelte";

    export let core;
    let parseEditor;
    let output = "";
    let errorLocation;

    function handle_change(event) {
        try {
            let parser = new core.EbnfParserParser(event.detail.value);
            output = parser.check("test");
            errorLocation = null;
        } catch (e) {
            console.log(e);
            output = e.kind + " at position " + e.position.start;
            errorLocation = {
                start: {
                    line: 1,
                    column: e.position.start + 1
                },
                end: {
                    line: 1,
                    column: e.position.end + 1
                }
            }
        }
    }
</script>

<style>
    .container {
        position: relative;
        width: 50%;
        height: 50%;
	}
	
    .container :global(section) {
        position: relative;
        padding: 42px 0 0 0;
        height: 100%;
        box-sizing: border-box;
    }
	
	.container :global(section) > :global(*):first-child {
        position: absolute;
        top: 0;
        left: 0;
        width: 100%;
        height: 42px;
        box-sizing: border-box;
    }
	
	.container :global(section) > :global(*):last-child {
        width: 100%;
        height: 100%;
    }
</style>

<div class="container">
    <CodeMirror bind:this="{parseEditor}" errorLocation="{errorLocation}" on:change={handle_change} />
</div>
<p>{output}</p>
