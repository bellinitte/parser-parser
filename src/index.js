import App from './site/App.svelte';

import('../Cargo.toml').then(async wasm => {
    const core = await wasm.default();
    new App({
        target: document.body,
        props: {
            core: core
        }
    });
});
