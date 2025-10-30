/**
 * Configuration Vite pour SvelteKit en mode SPA avec Tauri
 * 
 * Ce fichier configure Vite (l'outil de build) pour optimiser l'application :
 * - Mode SPA (Single Page Application) : pas de rendu côté serveur
 * - Optimisations pour Tauri : configuration spécifique pour les apps desktop
 * - Tree-shaking : suppression du code non utilisé pour réduire la taille
 */

import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
  plugins: [
    // Plugin SvelteKit : permet d'utiliser SvelteKit avec Vite
    sveltekit()
  ],

  // Configuration spécifique pour Tauri
  clearScreen: false, // Évite d'effacer la console lors du rechargement
  
  // Configuration du serveur de développement
  server: {
    port: 5173,        // Port par défaut de Vite
    strictPort: true,  // Échoue si le port n'est pas disponible
    host: 'localhost', // Écoute uniquement sur localhost pour la sécurité
    open: false,       // Désactive l'ouverture automatique du navigateur
  },

  // Variables d'environnement accessibles côté client
  envPrefix: ['VITE_', 'TAURI_'],

  // Optimisations de build pour la production
  build: {
    // Cible moderne pour de meilleures performances
    target: process.env.TAURI_PLATFORM == 'windows' ? 'chrome105' : 'safari13',
    
    // Minification agressive pour la production
    minify: !process.env.TAURI_DEBUG ? 'terser' : false,
    
    // Génère des source maps en mode debug uniquement
    sourcemap: !!process.env.TAURI_DEBUG,
    
    // Optimisations supplémentaires
    cssMinify: true,
    reportCompressedSize: false, // Désactive le rapport de taille (plus rapide)
    chunkSizeWarningLimit: 1000, // Augmente la limite d'avertissement
    
    // Configuration Terser pour une minification optimale
    terserOptions: {
      compress: {
        drop_console: !process.env.TAURI_DEBUG, // Supprime les console.log en production
        drop_debugger: true,
        pure_funcs: ['console.log', 'console.info', 'console.debug'],
        passes: 2 // Deux passes de compression
      },
      mangle: {
        safari10: true // Compatibilité Safari
      },
      format: {
        comments: false // Supprime tous les commentaires
      }
    },
    
    // Configuration du chunking pour optimiser le cache
    rollupOptions: {
      // Optimisations Rollup
      treeshake: {
        preset: 'recommended',
        moduleSideEffects: false
      }
    }
  }
});