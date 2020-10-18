<script>
    import CodeMirror from "./codemirror/CodeMirror.svelte";

    export let core;
    let parseEditor;
    let checkEditor;
    let parser;
    let output;;
    let error;

    function handle_parse_change(event) {
        try {
            parser = new core.EbnfParserParser(event.detail.value);
            error = null;
        } catch (e) {
            output = e.kind;
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

    function handle_check_change(event) {
        output = parser.check(event.detail.value);
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
        on:change="{handle_parse_change}"
    />
    <CodeMirror
        bind:this="{checkEditor}"
        on:change="{handle_check_change}"
    />
    <p>{output}</p>
</div>
