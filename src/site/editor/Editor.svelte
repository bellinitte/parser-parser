<script>
    import CodeMirror from "./codemirror/CodeMirror.svelte";

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
            error = {
                message: e.kind,
                from: {
                    line: e.span.from.line,
                    ch: e.span.from.ch,
                },
                to: {
                    line: e.span.to.line,
                    ch: e.span.to.ch,
                }
            };
        }
    }

    function lint() {
        let errors = [];
        if (error) {
            errors.push(error);
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
