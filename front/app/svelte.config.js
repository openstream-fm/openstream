//import adapter from '@sveltejs/adapter-auto';
import adapter from '@sveltejs/adapter-node';
import preprocess from 'svelte-preprocess';
import os from "os";
import child_process from "child_process";

const version = () => {
	if(os.hostname().includes("fedora")) {
		// for local development we use the date, as we may not yet commit the changes
		return Date.now().toString();
	} else {
		// for production we use the current commit hash
		return child_process.execSync('git rev-parse HEAD').toString().trim()
	}
		
}

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

		version: {
			name: version(),
			pollInterval: 1000,
		},

		files: {
			serviceWorker: "./src/service-worker.ts",
			errorTemplate: "./src/error.html",
			appTemplate: "./src/app.html",
			hooks: {
				server: "./src/hooks.server.ts"
			},
		},

		adapter: adapter(),
	}
};

export default config;
