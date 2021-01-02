<script>
    import { getContext } from "svelte";
    export let initialTheme;
    export let onSelect = () => {};
    export let onHover = () => {};

    const { close } = getContext("simple-modal");

    let selected = false;

    function handleSelect(event) {
        selected = true;
        disableButtons();
        const newTheme = event.target.innerText;
        onSelect(newTheme);
        close();
    }

    function disableButtons() {
        for (let li of document.getElementById("theme-list").children) {
            li.classList.add("disabled");
        }
    }

    function handleMouseEnter(event) {
        if (!selected) {
            const newTheme = event.target.innerText;
            onHover(newTheme);
        }
    }

    function handleMouseLeave() {
        if (!selected) {
            onHover(initialTheme);
        }
    }

    let themes = [
        "gruvbox dark",
        "gruvbox light",
        "monokai",
        "solarized dark",
        "solarized light",
        // "laser",
        // "dark magic girl",
        // "horizon",
        // "ishtar",
    ];
</script>

<style type="text/scss">
    @import "../styles/globals.scss";

    #theme-list {
        color: var(--fg-color);
        margin: 0;
        box-shadow: none;
        background-color: var(--bg-color);
        list-style-type: none;
        padding: 0;
        transition: color $transition-time;
        transition: background-color 0; // sync with the scrollbar
        cursor: default;

        > .theme {
            font-size: 14px;
            padding: 8px 16px;
            transition: color $transition-time;
            transition: background-color $transition-time;
        }

        > .theme:not(.disabled) {
            cursor: pointer;
        }

        > .theme:hover {
            background-color: var(--fg-color);
            color: var(--bg-color);

            // :global(body[theme="ishtar"]) & {
            //     color: var(--caret-color);
            // }
        }

        flex-grow: 1;
        overflow: auto;
    }

    #header,
    #footer {
        flex-grow: 0;
        width: 100%;
        min-height: 24px;
        background-color: var(--bg-color);
        transition: background-color 0; // sync with the scrollbar
    }
</style>

<div id="header"></div>
<ul id="theme-list">
    {#each themes as theme}
        <li
            class="theme"
            on:click="{handleSelect}"
            on:mouseenter="{handleMouseEnter}"
            on:mouseleave="{handleMouseLeave}"
            value="{theme}"
        >
            {theme}
        </li>
    {/each}
</ul>
<div id="footer"></div>
