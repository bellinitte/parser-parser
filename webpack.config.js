const path = require("path");
const CopyPlugin = require("copy-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const MiniCssExtractPlugin = require('mini-css-extract-plugin');

const mode = process.env.NODE_ENV || 'development';
const prod = mode === 'production';

const dist = path.resolve(__dirname, "dist");

module.exports = {
    mode,
    entry: {
        bundle: ["./src/index.js"]
    },
    resolve: {
        alias: {
            svelte: path.resolve('node_modules', 'svelte'),
            core: path.resolve(__dirname, 'pkg')
        },
        extensions: ['.mjs', '.js', '.svelte'],
        mainFields: ['svelte', 'browser', 'module', 'main']
    },
    output: {
        path: dist,
        filename: "[name].js"
    },
    devServer: {
        contentBase: dist,
    },
    module: {
        rules: [
            {
                test: /\.svelte$/,
                use: {
                    loader: 'svelte-loader',
                    options: {
                        emitCss: true,
                        hotReload: true
                    }
                }
            },
            {
                test: /\.css$/,
                use: [
                    prod ? MiniCssExtractPlugin.loader : 'style-loader',
                    'css-loader'
                ]
            }
        ]
    },
    plugins: [
        new MiniCssExtractPlugin({
            filename: '[name].css'
        }),
        
        new CopyPlugin([
            path.resolve(__dirname, "static")
        ]),

        new WasmPackPlugin({
            crateDirectory: __dirname,
        })
    ],
    devtool: prod ? false: 'source-map'
};
