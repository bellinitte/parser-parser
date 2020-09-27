<script>
    import { onMount, createEventDispatcher } from "svelte";
    const dispatch = createEventDispatcher();

    export let readonly = false;
    // export let error = null;
    export let flex = false;
    export let lineNumbers = true;
    export let tab = true;
    export let lint = null;

    let w;
    let h;
    let code = "";

    // We have to expose set and update methods, rather
    // than making this state-driven through props,
    // because it's difficult to update an editor
    // without resetting scroll otherwise
    export async function set(new_code) {
        code = new_code;
        updatingExternally = true;
        if (editor) {
            editor.setValue(code);
        }
        updatingExternally = false;
    }

    export function update(new_code) {
        code = new_code;
        if (editor) {
            const { left, top } = editor.getScrollInfo();
            editor.setValue((code = new_code));
            editor.scrollTo(left, top);
        }
    }

    export function resize() {
        editor.refresh();
    }

    export function focus() {
        editor.focus();
    }

    export function getHistory() {
        return editor.getHistory();
    }

    export function setHistory(history) {
        editor.setHistory(history);
    }

    export function clearHistory() {
        if (editor) editor.clearHistory();
    }

    let textAreaRef;
    let editor;
    let updatingExternally = false;
    let marker;
    let destroyed = false;
    let codeMirror;

    $: if (editor && w && h) {
        editor.refresh();
    }

    onMount(() => {
        (async () => {
            let mod = await import("./codemirror.js");
            codeMirror = mod.default;
            await createEditor("ebnf");
            if (editor) editor.setValue(code || "");
        })();

        return () => {
            destroyed = true;
            if (editor) editor.toTextArea();
        };
    });

    let first = true;

    async function createEditor(mode) {
        if (destroyed || !codeMirror) {
            return;
        }

        if (editor) {
            editor.toTextArea();
        }

        const opts = {
            lineNumbers,
            lineWrapping: true,
            indentWithTabs: true,
            indentUnit: 2,
            tabSize: 2,
            tabIndex: 4,
            value: "",
            mode: {
                name: mode,
            },
            readOnly: readonly,
            autoCloseBrackets: true,
            autoCloseTags: true,
            extraKeys: {
                Enter: "newlineAndIndentContinueMarkdownList",
                "Ctrl-/": "toggleComment",
                "Cmd-/": "toggleComment",
            },
            lint: lint
                ? {
                      getAnnotations: lint,
                      delay: Number.EPSILON,
                  }
                : false,
        };

        if (!tab) {
            opts.extraKeys["Tab"] = tab;
            opts.extraKeys["Shift-Tab"] = tab;
        }

        // Creating a text editor is a lot of work, so we yield
        // the main thread for a moment. This helps reduce jank
        if (first) {
            await sleep(50);
        }
        if (destroyed) {
            return;
        }

        editor = codeMirror.fromTextArea(textAreaRef, opts);
        editor.on("change", (instance) => {
            if (!updatingExternally) {
                const value = instance.getValue();
                dispatch("change", { value });
            }
        });

        if (first) {
            await sleep(50);
        }

        editor.refresh();
        first = false;
    }

    function sleep(ms) {
        return new Promise((resolve) => setTimeout(resolve, ms));
    }
</script>

<style>
    .codemirror-container {
        position: relative;
        width: 100%;
        height: 100%;
        border: none;
        line-height: 1.5;
        overflow: hidden;
    }

    .codemirror-container :global(.CodeMirror) {
        height: 100%;
        background: transparent;
        font: 400 16px/1.7;
        font-family: "JetBrains Mono", Consolas, monospace;
        color: var(--base);
    }

    .codemirror-container.flex :global(.CodeMirror) {
        height: auto;
    }

    .codemirror-container.flex :global(.CodeMirror-lines) {
        padding: 0;
    }

    .codemirror-container :global(.CodeMirror-gutters) {
        padding: 0 16px 0 8px;
        border: none;
    }

    .codemirror-container :global(.error-loc) {
        position: relative;
        border-bottom: 2px solid #da106e;
    }

    .codemirror-container :global(.error-line) {
        background-color: rgba(200, 0, 0, 0.05);
    }

    textarea {
        visibility: hidden;
    }
</style>

<div
    class="codemirror-container"
    class:flex
    bind:offsetWidth="{w}"
    bind:offsetHeight="{h}"
>
    <textarea bind:this="{textAreaRef}" readonly></textarea>
</div>
