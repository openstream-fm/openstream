import { sveltekit } from '@sveltejs/kit/vite';
import type { UserConfig } from 'vite';
import { searchForWorkspaceRoot } from "vite";
import { imagetools } from "vite-imagetools";

const config: UserConfig = {
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
		imagetools({
			force: true,
			removeMetadata: true,
		})
	]
};

export default config;
