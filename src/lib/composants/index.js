/**
 * Index des composants réutilisables
 * 
 * Ce fichier centralise l'export de tous les composants Svelte réutilisables
 * pour faciliter leur importation dans l'application.
 * 
 * Usage :
 * import { BoutonPrincipal, CarteContenu } from '$lib/composants';
 */

// Composants de base
export { default as BoutonPrincipal } from './BoutonPrincipal.svelte';
export { default as CarteContenu } from './CarteContenu.svelte';
export { default as IndicateurChargement } from './IndicateurChargement.svelte';

// Types TypeScript pour les props (si nécessaire)
/**
 * @typedef {'primary' | 'secondary' | 'ghost' | 'danger'} VarianteBouton
 * @typedef {'sm' | 'md' | 'lg'} TailleBouton
 * @typedef {'default' | 'primary' | 'success' | 'warning' | 'error'} VarianteCarte
 * @typedef {'sm' | 'md' | 'lg' | 'xl'} TailleIndicateur
 * @typedef {'spinner' | 'dots' | 'pulse' | 'bars'} TypeIndicateur
 */