<script>
    import CodeMirror from "./CodeMirror.svelte";

    export let core;
    let parseEditor;
    let output = "";
    let error;

    function handle_change(event) {
        try {
            let parser = new core.EbnfParserParser(event.detail.value);
            output = parser.check("test");
            error = null;
        } catch (e) {
            output = e.kind + " at position " + e.position.start;
            error = {
                message: e.kind,
                location: {
                    start: {
                        line: 1,
                        column: e.position.start + 1,
                    },
                    end: {
                        line: 1,
                        column: e.position.end + 1,
                    },
                }
            };
        }
    }

    function lint(text) {
        let errors = [];
        if (error) {
            errors.push({
                from: { line: error.location.start.line - 1, ch: error.location.start.column - 1},
                to: { line: error.location.end.line - 1, ch: error.location.end.column - 1},
                message: error.message
            })
        }
        return errors;
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
    <CodeMirror
        bind:this="{parseEditor}"
        lint="{lint}"
        on:change="{handle_change}"
    />
</div>
