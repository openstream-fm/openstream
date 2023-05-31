import { sveltekit } from '@sveltejs/kit/vite';
import type { UserConfigFn } from 'vite';
import { searchForWorkspaceRoot } from "vite";
import { imagetools } from "vite-imagetools";

const config: UserConfigFn = (ctx) => {

	return {
		resolve: {
			dedupe: ["@sveltejs/kit", "svelte", "@mdi/js", "http-status-codes", "kleur"],
		},

		server: {
			https: false,
			host: "0.0.0.0",
			port: 3001,
			strictPort: true,
			proxy: {
				"/api": "https://studio.local.openstream.fm",
			},

			fs: {
				allow: [
					searchForWorkspaceRoot(process.cwd()),
					"../share/src",
				]
			}
		},

		// @ts-ignore
		plugins: [

			sveltekit(),

			// {
			// 	name: "isomorphic-apexchars",
			// 	config: (config, env) => {
			// 		// for(let i = 0; i < 250000; i++) {
			// 		// 	console.log(`target ${i}: ${config.build?.target}`);
			// 		// }
			// 		if (String(config.build?.target).startsWith("node")) {
			// 			return {
			// 				...config,
			// 				alias: {
			// 					resolve: {
			// 						...config.resolve,
			// 						alias: {
			// 							...config.resolve?.alias,
			// 							"apexcharts": "$share/apexcharts.ssr.ts",
			// 						}
			// 					}
			// 				}
			// 			}
			// 		} else {
			// 			return null;
			// 		}
			// 	}
			// },

			imagetools({
				force: true,
				removeMetadata: true,
			}),

		]
	};
}

export default config;
