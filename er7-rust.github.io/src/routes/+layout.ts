// Every route is prerendered to static HTML for GitHub Pages, and every
// internal link carries a trailing slash so a path resolves to its own
// directory index rather than relying on server-side redirects.
export const prerender = true;
export const trailingSlash = 'always';
