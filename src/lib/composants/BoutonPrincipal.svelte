<!--
  Composant BoutonPrincipal - Bouton réutilisable avec variants
  
  Props:
  - texte: string - Texte affiché sur le bouton
  - variante: 'primary' | 'secondary' | 'ghost' | 'danger' - Style du bouton
  - taille: 'sm' | 'md' | 'lg' - Taille du bouton
  - desactive: boolean - État désactivé
  - chargement: boolean - État de chargement
  - icone: string - Icône à afficher (emoji ou texte)
  - type: 'button' | 'submit' | 'reset' - Type HTML du bouton
  
  Événements:
  - on:click - Clic sur le bouton
  - on:focus - Focus sur le bouton
  - on:blur - Perte de focus
-->

<script>
  import { createEventDispatcher } from 'svelte';
  
  // Props avec valeurs par défaut
  export let texte = '';
  export let variante = 'primary';
  export let taille = 'md';
  export let desactive = false;
  export let chargement = false;
  export let icone = '';
  export let type = 'button';
  export let largeurComplete = false;
  
  // Dispatcher pour les événements
  const dispatch = createEventDispatcher();
  
  // Classes CSS calculées
  $: classes = [
    'btn',
    `btn-${variante}`,
    `btn-${taille}`,
    largeurComplete ? 'btn-full' : '',
    chargement ? 'btn-loading' : '',
    desactive ? 'btn-disabled' : ''
  ].filter(Boolean).join(' ');
  
  // Gestionnaires d'événements
  function gererClic(event) {
    if (!desactive && !chargement) {
      dispatch('click', event);
    }
  }
  
  function gererFocus(event) {
    dispatch('focus', event);
  }
  
  function gererBlur(event) {
    dispatch('blur', event);
  }
</script>

<button
  class={classes}
  {type}
  disabled={desactive || chargement}
  on:click={gererClic}
  on:focus={gererFocus}
  on:blur={gererBlur}
  aria-busy={chargement}
  aria-disabled={desactive}
>
  {#if chargement}
    <span class="btn-spinner" aria-hidden="true"></span>
    <span class="sr-only">Chargement...</span>
  {:else if icone}
    <span class="btn-icon" aria-hidden="true">{icone}</span>
  {/if}
  
  {#if texte}
    <span class="btn-text">{texte}</span>
  {/if}
  
  <!-- Slot pour contenu personnalisé -->
  <slot />
</button>

<style>
  .btn {
    /* Base du bouton */
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: var(--espacement-sm);
    font-family: inherit;
    font-weight: 500;
    line-height: 1.5;
    text-align: center;
    text-decoration: none;
    vertical-align: middle;
    cursor: pointer;
    user-select: none;
    border: 1px solid transparent;
    border-radius: var(--rayon-bordure-md);
    transition: all var(--transition-rapide);
    position: relative;
    overflow: hidden;
    white-space: nowrap;
  }
  
  .btn:focus {
    outline: 2px solid var(--couleur-bordure-focus);
    outline-offset: 2px;
  }
  
  .btn:disabled {
    opacity: 0.6;
    cursor: not-allowed;
    pointer-events: none;
  }
  
  /* === Variants === */
  
  /* Bouton principal */
  .btn-primary {
    background: linear-gradient(135deg, var(--couleur-principale) 0%, var(--couleur-secondaire) 100%);
    color: white;
    border-color: var(--couleur-principale);
    box-shadow: var(--ombre-principale);
  }
  
  .btn-primary:hover:not(:disabled) {
    background: linear-gradient(135deg, var(--couleur-principale-claire) 0%, var(--couleur-secondaire-claire) 100%);
    transform: translateY(-1px);
    box-shadow: var(--ombre-lg);
  }
  
  .btn-primary:active:not(:disabled) {
    transform: translateY(0);
    box-shadow: var(--ombre-md);
  }
  
  /* Bouton secondaire */
  .btn-secondary {
    background: var(--couleur-fond-tertiaire);
    color: var(--couleur-texte-principal);
    border-color: var(--couleur-bordure);
  }
  
  .btn-secondary:hover:not(:disabled) {
    background: var(--couleur-fond-quaternaire);
    border-color: var(--couleur-bordure-claire);
    transform: translateY(-1px);
  }
  
  /* Bouton fantôme */
  .btn-ghost {
    background: transparent;
    color: var(--couleur-texte-secondaire);
    border-color: transparent;
  }
  
  .btn-ghost:hover:not(:disabled) {
    background: var(--couleur-fond-tertiaire);
    color: var(--couleur-texte-principal);
    border-color: var(--couleur-bordure);
  }
  
  /* Bouton danger */
  .btn-danger {
    background: linear-gradient(135deg, var(--couleur-erreur) 0%, #dc2626 100%);
    color: white;
    border-color: var(--couleur-erreur);
    box-shadow: var(--ombre-erreur);
  }
  
  .btn-danger:hover:not(:disabled) {
    background: linear-gradient(135deg, var(--couleur-erreur-claire) 0%, var(--couleur-erreur) 100%);
    transform: translateY(-1px);
    box-shadow: var(--ombre-lg);
  }
  
  /* === Tailles === */
  
  .btn-sm {
    padding: var(--espacement-xs) var(--espacement-md);
    font-size: var(--taille-police-xs);
    gap: var(--espacement-xs);
  }
  
  .btn-md {
    padding: var(--espacement-sm) var(--espacement-lg);
    font-size: var(--taille-police-sm);
    gap: var(--espacement-sm);
  }
  
  .btn-lg {
    padding: var(--espacement-md) var(--espacement-xl);
    font-size: var(--taille-police-base);
    gap: var(--espacement-md);
  }
  
  /* === Modificateurs === */
  
  .btn-full {
    width: 100%;
  }
  
  .btn-loading {
    pointer-events: none;
  }
  
  .btn-disabled {
    opacity: 0.5;
    cursor: not-allowed;
    pointer-events: none;
  }
  
  /* === Éléments internes === */
  
  .btn-spinner {
    width: 1em;
    height: 1em;
    border: 2px solid transparent;
    border-top: 2px solid currentColor;
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }
  
  .btn-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.1em;
  }
  
  .btn-text {
    flex: 1;
  }
  
  @keyframes spin {
    0% { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
  }
  
  /* === Responsive === */
  @media (max-width: 768px) {
    .btn-lg {
      padding: var(--espacement-sm) var(--espacement-lg);
      font-size: var(--taille-police-sm);
    }
    
    .btn-md {
      padding: var(--espacement-xs) var(--espacement-md);
      font-size: var(--taille-police-xs);
    }
  }
  
  /* === Accessibilité === */
  @media (prefers-reduced-motion: reduce) {
    .btn {
      transition: none;
    }
    
    .btn-spinner {
      animation: none;
    }
    
    .btn:hover:not(:disabled) {
      transform: none;
    }
  }
</style>