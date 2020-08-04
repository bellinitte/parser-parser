import svelte from 'rollup-plugin-svelte';
import resolve from '@rollup/plugin-node-resolve';
import commonjs from '@rollup/plugin-commonjs';
import livereload from 'rollup-plugin-livereload';
import { terser } from 'rollup-plugin-terser';
import wasm from "@wasm-tool/rollup-plugin-rust";
import sveltePreprocess from 'svelte-preprocess';
import typescript from '@rollup/plugin-typescript';
import copy from 'rollup-plugin-copy';
import alias from '@rollup/plugin-alias';
import serve from './scripts/serve';

const production = !process.env.ROLLUP_WATCH;

export default {
    input: {
        site: 'src/index.ts',
    },
    output: {
        format: 'iife',
        name: 'app',
        dir: 'dist',
        sourcemap: !production
    },
    plugins: [
        copy({
            targets: [
                { src: 'public/*', dest: 'dist' }
            ]
        }),
        alias({
            entries: [
                { find: 'core', replacement: './Cargo.toml' }
            ]
        }),
        svelte({
            dev: !production,
			preprocess: sveltePreprocess(),
        }),
        resolve({
            browser: true,
            dedupe: ['svelte']
        }),
        commonjs(),
        wasm({
            debug: !production,
            verbose: !production,
        }),
		typescript({
            moduleResolution: "node",
            target: "es2017",
            /** 
             Svelte Preprocess cannot figure out whether you have a value or a type, so tell TypeScript
            to enforce using `import type` instead of `import` for Types.
            */
            importsNotUsedAsValues: "error",
            /** Requests the runtime types from the svelte modules by default */
            types: ["svelte"],
            // module: "ESNext",
            allowSyntheticDefaultImports: true,
            sourceMap: !production,
            baseUrl: ".",
            paths: {
                core: ["target/wasm-pack/parser-parser-core/index.js"],
            },
            include: ["src/**/*"],
            exclude: ["node_modules/*"],
        }),

        !production && serve(),
        !production && livereload('public'),
        production && terser()
    ],
    watch: {
        clearScreen: false
    }
};
