import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
  plugins: [
    sveltekit({
      preprocess: vitePreprocess(),
      // Every page is prerendered to static HTML and served by GitHub Pages,
      // so there is no server at runtime. `fallback` is omitted on purpose:
      // with `strict: true` the build fails if a route cannot be prerendered,
      // which is the signal we want rather than a silent client-side fallback.
      adapter: adapter({ pages: 'build', assets: 'build', strict: true })
    })
  ]
});
