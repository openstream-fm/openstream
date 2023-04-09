//import adapter from '@sveltejs/adapter-auto';
import adapter from '@sveltejs/adapter-node';
import preprocess from 'svelte-preprocess';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	// Consult https://github.com/sveltejs/svelte-preprocess
	// for more information about preprocessors
	preprocess: preprocess(),

	kit: {
		serviceWorker: {
			register: false,
		},

		alias: {
			"$server": "../server/src",
			"$share": "../share/src",
		},

		files: {
			serviceWorker: "./src/service-worker.ts",
			hooks: {
				server: "./src/hooks.server.ts"
			},
		},
		
		adapter: adapter(),
	}
};

export default config;
