<script>
    import CodeMirror from "./codemirror/CodeMirror.svelte";
    import ParseTree from "./ParseTree.svelte";

    export let core;
    let parseEditor;
    let checkEditor;
    let parser;
    let productionRules = [];
    let initialProductionRule;
    let output = null;
    let error;

    async function handleParseChange(event) {
        parser = undefined;
        output = null;
        try {
            parser = new core.EbnfParserParser(event.detail.value);
            productionRules = parser.productionRules;
            if (initialProductionRule === null || !productionRules.includes(initialProductionRule)) {
                initialProductionRule = productionRules[0];
            }
            error = null;
            check(checkEditor.get());
        } catch (e) {
            console.error(e);
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

    function handleCheckChange(event) {
        check(event.detail.value);
    }

    function handleInitialProductionRuleChange(event) {
        check(checkEditor.get());
    }

    function check(input) {
        if (parser) {
            output = parser.check(input, initialProductionRule);
            console.log(output);
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

<main>
    <div id="left" class="editor">
        <div class="bordered-editor">
            <CodeMirror
                bind:this="{parseEditor}"
                lint="{lint}"
                on:change="{handleParseChange}"
            />
        </div>
    </div>
    <div id="right" class="editor">
        <!-- svelte-ignore a11y-no-onchange -->
        <select
            bind:value={initialProductionRule} 
            on:change={handleInitialProductionRuleChange}
            disabled={!parser}
            style="width: 300px;"
        >
            {#each productionRules as productionRule}
                <option value={productionRule}>
                    {productionRule}
                </option>
            {/each}
        </select>
        <div class="bordered-editor" style="margin-bottom: 8px; height: 200px;">
            <CodeMirror
                bind:this="{checkEditor}"
                on:change="{handleCheckChange}"
                mode="text"
            />
        </div>
        <div class="bordered-editor">
            {#if output != null}
                <ParseTree tree={output} />
            {:else}
                <p class="output">failure</p>
            {/if}
        </div>
    </div>
</main>

<style>
    main {
        /* display: flex;
        flex-direction: row; */
        position: relative;
        width: 100%;
        height: 100%;
        display: inline-block;
    }

    #left {
        width: 50%;
        height: 520px;
        float: left;
    }
    
    #right {
        width: 50%;
        float: right;
        /* width: 50%; */
    }

    .editor {
        padding: 8px;
        box-sizing: border-box;
    }

    .bordered-editor {
        box-sizing: border-box;
        padding: 8px;
        border-radius: 4px;
        background-color: rgb(40, 40, 40);
        /* border: 1px solid rgb(60, 60, 60); */
        width: 100%;
        height: 100%;
    }

    .output {
        font-family: "JetBrains Mono", Consolas, monospace;
        font-size: 16px;
        color: #7c7977;
    }
</style>
