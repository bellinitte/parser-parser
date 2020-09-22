import svelte from "rollup-plugin-svelte";
import resolve from "@rollup/plugin-node-resolve";
import commonjs from "@rollup/plugin-commonjs";
import livereload from "rollup-plugin-livereload";
import { terser } from "rollup-plugin-terser";
import wasm from "@wasm-tool/rollup-plugin-rust";
import copy from "rollup-plugin-copy";
import alias from "@rollup/plugin-alias";
import serve from "./scripts/serve";
import css from "rollup-plugin-css-only";

const production = !process.env.ROLLUP_WATCH;

export default {
    input: {
        bundle: "src/index.js",
    },
    output: {
        format: "es",
        name: "app",
        dir: "dist",
        sourcemap: !production,
    },
    plugins: [
        css({
            output: "dist/extra.css",
        }),
        copy({
            targets: [{ src: "public/*", dest: "dist" }],
        }),
        alias({
            entries: [{ find: "core", replacement: "./Cargo.toml" }],
        }),
        svelte({
            dev: !production,
        }),
        resolve({
            browser: true,
            dedupe: ["svelte"],
        }),
        commonjs(),
        wasm({
            debug: !production,
            verbose: !production,
        }),

        !production && serve(),
        !production && livereload("dist"),
        production && terser(),
    ],
    watch: {
        clearScreen: false,
    },
};
