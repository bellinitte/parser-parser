<script>
    import CodeMirror from "./codemirror/CodeMirror.svelte";

    export let core;
    let parseEditor;
    let checkEditor;
    let parser;
    let output = "";
    let error;

    async function handle_parse_change(event) {
        parser = undefined;
        output = "";
        try {
            parser = new core.EbnfParserParser(event.detail.value);
            error = null;
            output = parser.check(checkEditor.get(), parser.productionRules[0]) ? "success" : "failure";
        } catch (e) {
            // console.error(e);
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
        if (parser) {
            output = parser.check(event.detail.value, parser.productionRules[0]) ? "success" : "failure";
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
        width: 100%;
        height: 100%;
        display: inline-block;
    }

    .editor-left {
        width: 50%;
        height: 100%;
        float: left;
    }
    
    .editor-right {
        width: 50%;
        height: 15%;
        float: left;
    }

    .output {
        float: left;
        font-family: "JetBrains Mono", Consolas, monospace;
        color: #928374;
        padding-left: 24px;
    }
</style>

<div class="container">
    <div class="editor-left">
        <CodeMirror
            bind:this="{parseEditor}"
            lint="{lint}"
            on:change="{handle_parse_change}"
        />
    </div>
    <div class="editor-right">
        <CodeMirror
            bind:this="{checkEditor}"
            on:change="{handle_check_change}"
            mode="text"
        />
    </div>
    {#if output != ""}
        <p class="output">{"> " + output}</p>
    {/if}
</div>
