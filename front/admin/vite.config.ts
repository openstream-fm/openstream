import { sveltekit } from '@sveltejs/kit/vite';
import type { UserConfig } from 'vite';

const config: UserConfig = {
	resolve: {
		dedupe: ["@sveltejs/kit", "svelte", "@mdi/js", "http-status-codes", "kleur"],
	},
	server: {
		https: false,
		host: "0.0.0.0",
		port: 3002,
		strictPort: true,
		proxy: {
			"/api": "https://admin.local.openstream.fm",
		} 
	},
	// @ts-ignore
	plugins: [sveltekit()]
};

export default config;
