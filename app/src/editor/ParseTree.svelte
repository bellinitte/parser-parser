<script>
    import Tree from "svelte-tree";

    export let tree;
</script>

<main>
    <Tree tree={[tree]} let:node>
        <div class="node">{node.name}</div>
    </Tree>
</main>

<style>
    .node {
        float: left;
        font-family: "JetBrains Mono", Consolas, monospace;
        color: #928374;
        padding-left: 24px;
    }
</style>
