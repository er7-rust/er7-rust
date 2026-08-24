import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
  preprocess: vitePreprocess(),
  kit: {
    // Every page is prerendered to static HTML and served by GitHub Pages,
    // so there is no server at runtime. `fallback` is omitted on purpose:
    // with `strict: true` the build fails if a route cannot be prerendered,
    // which is the signal we want rather than a silent client-side fallback.
    adapter: adapter({
      pages: 'build',
      assets: 'build',
      strict: true
    })
    // No `paths.base`: this repo is `<org>.github.io`, so the site is served
    // from the domain root. A project-pages repo would need `base` set to
    // the repository name here.
  }
};

export default config;
