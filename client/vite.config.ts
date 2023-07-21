// vite.config.js

import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

/** @type {import('vite').UserConfig} */
export default ({ mode }) => {
	let devEnvSettings = {};
	if (mode === 'development') {
		devEnvSettings = {
			server: {
				port: 3000,
				proxy: {
					'/api': 'http://127.0.0.1:8080'
				}
			},
		}
	}
	return defineConfig({
		plugins: [sveltekit()],
		...devEnvSettings,

	});
};