<script lang="ts">
    import moment from 'moment';
    console.log(moment().format());

    import {onMount} from "svelte";
    import wasm from "core";

    console.log(wasm);

    let core = null;
    onMount(async () => {
        core = await wasm();
        console.log(core);
    });
</script>

<style>
    h1 {
        color: purple;
    }
</style>

{#if core}
<h1>Hello {core.getMessage()}!</h1>
{/if}
