import { cloudflare } from '@cloudflare/vite-plugin';
import { paraglideVitePlugin } from '@inlang/paraglide-js';
import babel from '@rolldown/plugin-babel';
import tailwindcss from '@tailwindcss/vite';
import { devtools } from '@tanstack/devtools-vite';
import { tanstackStart } from '@tanstack/react-start/plugin/vite';
import react, { reactCompilerPreset } from '@vitejs/plugin-react';
import { nitro } from 'nitro/vite';
import { defineConfig } from 'vite';

const config = defineConfig({
	resolve: { tsconfigPaths: true },
	plugins: [
		devtools(),
		paraglideVitePlugin({
			project: './project.inlang',
			outdir: './src/paraglide',
			strategy: ['cookie', 'preferredLanguage', 'baseLocale'],
			// cookieName: 'PARAGLIDE_LOCALE',
			routeStrategies: [{ match: '/api/:path(.*)?', exclude: true }],
		}),
		// cloudflare({ viteEnvironment: { name: 'ssr' } }),
		nitro({ preset: 'cloudflare-module', cloudflare: {} }),
		tailwindcss(),
		tanstackStart(),
		react(),
		babel({
			presets: [reactCompilerPreset()],
		}),
	],
	environments: {
		ssr: { build: { rollupOptions: { input: './src/server.ts' } } },
	},
});

export default config;
