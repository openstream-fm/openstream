import adapter from '@sveltejs/adapter-node';
import preprocess from 'svelte-preprocess';
// import os from "os";
// import child_process from "child_process";

// const version = () => {
// 	// TODO: change this to a better way of knowing if this is a local build
// 	if(os.hostname().includes("fedora")) {
// 		// for local development we use the date, as we may not yet commit the changes
// 		return Date.now().toString();
// 	} else {
// 		// for production we use the current commit hash
// 		return child_process.execSync('git rev-parse HEAD').toString().trim()
// 	}
// }

const version = () => Date.now().toString();

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
			"$static": "../static/studio/static",
			"$defs": "../server/src/defs",
			"$api": "../server/src/defs/api",
		},

		version: {
			name: version(),
		},

		files: {
			serviceWorker: "./src/service-worker.ts",
			errorTemplate: "./src/error.html",
			appTemplate: "./src/app.html",
			assets: "../static/studio",
			hooks: {
				server: "./src/hooks.server.ts"
			},
		},

		paths: {
			relative: false,
		},

		adapter: adapter(),
	}
};

export default config;
