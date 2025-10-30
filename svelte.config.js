/**
 * Configuration SvelteKit pour mode SPA (Single Page Application)
 * 
 * Configuration simplifiée pour Tauri :
 * - Génération de fichiers statiques uniquement
 * - Optimisé pour les applications desktop Tauri
 */

import adapter from '@sveltejs/adapter-static';

/** @type {import('@sveltejs/kit').Config} */
const config = {
  kit: {
    // Adaptateur statique : génère des fichiers HTML/CSS/JS statiques
    adapter: adapter({
      pages: 'build',
      assets: 'build',
      fallback: 'index.html', // SPA fallback pour le routing côté client
      precompress: true,      // Pré-compression gzip/brotli pour de meilleures performances
      strict: true           // Mode strict pour détecter les erreurs de build
    }),
    
    // Optimisations supplémentaires
    prerender: {
      handleHttpError: 'warn',
      handleMissingId: 'warn'
    },
    
    // Configuration des alias pour de meilleures performances de build
    alias: {
      $components: 'src/lib/composants',
      $styles: 'src/lib/styles',
      $utils: 'src/lib/utils'
    }
  }
};

export default config;