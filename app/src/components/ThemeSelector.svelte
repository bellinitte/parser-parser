<script>
    import { getContext } from "svelte";
    import ThemeDialog from "./ThemeDialog.svelte";

    const { open } = getContext("simple-modal");

    export let value;

    let label = value;

    const onSelect = (selectedTheme) => {
        value = selectedTheme;
        label = selectedTheme;
    };

    const onHover = (selectedTheme) => {
        value = selectedTheme;
    };

    const showDialog = () => {
        open(
            ThemeDialog,
            {
                initialTheme: value,
                onSelect,
                onHover,
            },
            {
                closeButton: false,
                closeOnEsc: true,
                closeOnOuterClick: true,
                styleContent: {
                    padding: 0,
                    display: "flex",
                    flexDirection: "column",
                    flexWrap: "nowrap",
                    borderRadius: "8px",
                },
                styleWindow: {
                    background: "none",
                },
            }
        );
    };
</script>

<style type="text/scss">
    @import "../styles/globals.scss";

    button {
        font-family: inherit;
        font-size: inherit;
        margin: 0 0 0.5em 0;
        border: none;
        color: var(--comment-color);
        background: none;
        outline: none;
        transition: color $transition-time;
        cursor: pointer;

        &:hover,
        &:active,
        &:focus {
            color: var(--fg-color);

            #icon-path {
                fill: var(--fg-color);
            }
        }

        display: flex;

        #icon-svg {
            margin-right: 4px;
        }

        #icon-path {
            transition: fill $transition-time;
            fill: var(--comment-color);
        }
    }
</style>

<button on:click="{showDialog}">
    <svg id="icon-svg" width="24" height="24">
        <path
            id="icon-path"
            d="M12 3c-4.97 0-9 4.03-9 9s4.03 9 9 9c.83 0 1.5-.67 1.5-1.5 0-.39-.15-.74-.39-1.01-.23-.26-.38-.61-.38-.99 0-.83.67-1.5 1.5-1.5H16c2.76 0 5-2.24 5-5 0-4.42-4.03-8-9-8zm-5.5 9c-.83 0-1.5-.67-1.5-1.5S5.67 9 6.5 9 8 9.67 8 10.5 7.33 12 6.5 12zm3-4C8.67 8 8 7.33 8 6.5S8.67 5 9.5 5s1.5.67 1.5 1.5S10.33 8 9.5 8zm5 0c-.83 0-1.5-.67-1.5-1.5S13.67 5 14.5 5s1.5.67 1.5 1.5S15.33 8 14.5 8zm3 4c-.83 0-1.5-.67-1.5-1.5S16.67 9 17.5 9s1.5.67 1.5 1.5-.67 1.5-1.5 1.5z"
        ></path>
    </svg>{label}
</button>
