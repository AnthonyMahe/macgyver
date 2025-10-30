

export const index = 0;
let component_cache;
export const component = async () => component_cache ??= (await import('../entries/pages/_layout.svelte.js')).default;
export const universal = {
  "ssr": false,
  "prerender": false
};
export const universal_id = "src/routes/+layout.js";
export const imports = ["_app/immutable/nodes/0.XNcZ_1o_.js","_app/immutable/chunks/Djf__QTE.js","_app/immutable/chunks/B6yj-TJc.js","_app/immutable/chunks/B93ebTeG.js","_app/immutable/chunks/BAAQZNEU.js"];
export const stylesheets = ["_app/immutable/assets/0.CH4srT2M.css"];
export const fonts = [];
