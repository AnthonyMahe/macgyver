

export const index = 1;
let component_cache;
export const component = async () => component_cache ??= (await import('../entries/fallbacks/error.svelte.js')).default;
export const imports = ["_app/immutable/nodes/1.C5yzOSVW.js","_app/immutable/chunks/Djf__QTE.js","_app/immutable/chunks/B6yj-TJc.js","_app/immutable/chunks/B93ebTeG.js"];
export const stylesheets = [];
export const fonts = [];
