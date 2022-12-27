import { sveltekit } from '@sveltejs/kit/vite';
import type { UserConfig } from 'vite';

const config: UserConfig = {
	server: {
		hmr: {
			clientPort: 3100,
		}
	},
	plugins: [sveltekit()]
};

export default config;
