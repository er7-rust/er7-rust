import type { PageLoad } from './$types';

// page.data.title convention: the layout's SharePicker reads this back
// via page.data, so the share title and the tab title (in +page.svelte's
// <svelte:head>) are one string, not two that can drift.
export const load: PageLoad = () => ({
  title: 'serde-er7 — Serde support for the ER7 message tree'
});
