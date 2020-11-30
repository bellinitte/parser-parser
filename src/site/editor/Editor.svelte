<script>
    import CodeMirror from "./codemirror/CodeMirror.svelte";
    import Tree from "svelte-tree";

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
            on:change="{handleParseChange}"
        />
    </div>
    <!-- svelte-ignore a11y-no-onchange -->
    <select
        bind:value={initialProductionRule} 
        on:change={handleInitialProductionRuleChange}
        disabled={!parser}
    >
		{#each productionRules as productionRule}
			<option value={productionRule}>
				{productionRule}
			</option>
		{/each}
	</select>
    <div class="editor-right">
        <CodeMirror
            bind:this="{checkEditor}"
            on:change="{handleCheckChange}"
            mode="text"
        />
    </div>
    {#if output != null}
        <Tree tree={[output]} let:node>
            <div class="output">{node.name}</div>
        </Tree>
        <!-- <p class="output">{"> " + output}</p> -->
    {/if}
</div>
