const sveltePreprocess = require('svelte-preprocess');

module.exports = {
	transform: {
		'^.+\\.js$': 'babel-jest',
		'^.+\\.svelte$': [
			'svelte-jester',
			{
				preprocess: true,
				debug: true,
				noStyles: true,
				compilerOptions: {}
			}
		]
	},
	transformIgnorePatterns: ['/(?!pkg)'],
	moduleFileExtensions: ['js', 'svelte', 'wasm'],
	coverageReporters: ['html'],
	bail: false,
	verbose: true,
	roots: ["tests"]
};
