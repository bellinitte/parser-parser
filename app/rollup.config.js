import svelte from "rollup-plugin-svelte";
import resolve from "@rollup/plugin-node-resolve";
import commonjs from "@rollup/plugin-commonjs";
import { terser } from "rollup-plugin-terser";
import preprocess from "svelte-preprocess";
import copy from "rollup-plugin-copy";
import serve from "./scripts/serve";

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
        chunkFileNames: "[name].js",
    },
    plugins: [
        copy({
            targets: [{ src: "public/*", dest: "dist" }],
        }),
        svelte({
            dev: !production,
            css: (css) => {
                css.write("dist/bundle.css", !production);
            },
            preprocess: preprocess(),
        }),
        resolve({
            browser: true,
            dedupe: ["svelte"],
        }),
        commonjs(),

        !production && serve(),
        production && terser(),
    ],
    watch: {
        clearScreen: false,
    },
};
