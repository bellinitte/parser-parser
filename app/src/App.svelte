<script>
    import { Router, Link, Route } from "svelte-routing";
    import Modal from "svelte-simple-modal";
    import Page from "./routes/Page.svelte";
    import Info from "./routes/Info.svelte";

    export let theme;
    export let url = "";

    const body = document.getElementsByTagName("body")[0];

    window.addEventListener("load", () => {
        body.classList.add("loaded");
    });

    $: {
        body.setAttribute("theme", theme);
        localStorage.setItem("theme", theme);
    }
</script>

<style type="text/scss">
    @font-face {
        font-family: "JetBrains Mono";
        src: url("fonts/JetBrainsMono-Regular.woff") format("woff");
    }

    @import "./styles/themes.scss";
    @import "./styles/globals.scss";

    :global(body) {
        font-family: "JetBrains Mono", Consolas, monospace;
        background-color: var(--bg-color) !important;

        &.loaded {
            transition: background-color $transition-time;
        }
    }

    :global(html),
    :global(body),
    div {
        width: 100%;
        height: 100%;
        margin: 0;
    }

    // Scrollbars

    :global(body, body *) {
        scrollbar-width: thin;
        scrollbar-color: var(--accent-color) var(--bg-color);
    }

    :global(::-webkit-scrollbar) {
        width: 6px;
        height: 6px;
    }

    :global(::-webkit-scrollbar-track) {
        background-color: var(--bg-color);
    }

    :global(::-webkit-scrollbar-thumb) {
        background: var(--accent-color);
        border-radius: 3px !important;
    }

    :global(::-webkit-scrollbar-thumb:hover) {
        background: var(--fg-color);
    }
</style>

<Modal>
    <Router url="{url}">
        <!-- <nav>
            <Link to="/">Home</Link>
            <Link to="/info">Info</Link>
        </nav> -->
        <Route path="/info">
            <Info />
        </Route>
        <Route path="/">
            <Page bind:theme />
        </Route>
    </Router>
</Modal>
