import { sveltekit } from '@sveltejs/kit/vite';
import type { UserConfig } from 'vite';

const config: UserConfig = {
	resolve: {
		dedupe: ["@sveltejs/kit", "svelte", "@mdi/js"],
	},
	server: {
		hmr: {
			clientPort: 3100,
		}
	},
	plugins: [sveltekit()]
};

export default config;
