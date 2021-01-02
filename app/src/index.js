import App from "./App.svelte";

new App({
    target: document.body,
    props: {
        theme: localStorage.getItem("theme") || "gruvbox dark",
    },
});
