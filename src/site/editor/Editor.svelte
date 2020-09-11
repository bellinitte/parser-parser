<script>
	import CodeMirror from './CodeMirror.svelte';

    export let input;
    let errorLoc;
</script>

<style>
	.container {
		position: relative;
		width: 100%;
		height: 100%;
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
        bind:this={input}
        {errorLoc}
    />
</div>
