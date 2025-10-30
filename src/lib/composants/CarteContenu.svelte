<!--
  Composant CarteContenu - Carte réutilisable pour afficher du contenu structuré
  
  Props:
  - titre: string - Titre de la carte
  - sousTitre: string - Sous-titre optionnel
  - icone: string - Icône à afficher dans l'en-tête
  - variante: 'default' | 'primary' | 'success' | 'warning' | 'error' - Style de la carte
  - cliquable: boolean - Si la carte est cliquable
  - chargement: boolean - État de chargement
  - elevation: 'sm' | 'md' | 'lg' - Niveau d'ombre
  
  Slots:
  - header: Contenu personnalisé pour l'en-tête
  - default: Contenu principal de la carte
  - footer: Contenu du pied de page
  - actions: Actions (boutons) à afficher
  
  Événements:
  - on:click - Clic sur la carte (si cliquable)
-->

<script>
  import { createEventDispatcher } from 'svelte';
  
  // Props avec valeurs par défaut
  export let titre = '';
  export let sousTitre = '';
  export let icone = '';
  export let variante = 'default';
  export let cliquable = false;
  export let chargement = false;
  export let elevation = 'md';
  
  // Dispatcher pour les événements
  const dispatch = createEventDispatcher();
  
  // Classes CSS calculées
  $: classes = [
    'carte',
    `carte-${variante}`,
    `carte-elevation-${elevation}`,
    cliquable ? 'carte-cliquable' : '',
    chargement ? 'carte-chargement' : ''
  ].filter(Boolean).join(' ');
  
  // Gestionnaire de clic
  function gererClic(event) {
    if (cliquable && !chargement) {
      dispatch('click', event);
    }
  }
</script>

<!-- svelte-ignore a11y-no-noninteractive-tabindex -->
<div
  class={classes}
  role={cliquable ? 'button' : 'article'}
  tabindex={cliquable ? 0 : -1}
  on:click={gererClic}
  on:keydown={(e) => {
    if (cliquable && (e.key === 'Enter' || e.key === ' ')) {
      e.preventDefault();
      gererClic(e);
    }
  }}
  aria-busy={chargement}
>
  <!-- Indicateur de chargement -->
  {#if chargement}
    <div class="carte-chargement-overlay">
      <div class="carte-spinner"></div>
    </div>
  {/if}
  
  <!-- En-tête de la carte -->
  {#if $$slots.header || titre || icone}
    <div class="carte-header">
      <slot name="header">
        <div class="carte-header-content">
          {#if icone}
            <div class="carte-icone" aria-hidden="true">{icone}</div>
          {/if}
          
          <div class="carte-header-text">
            {#if titre}
              <h3 class="carte-titre">{titre}</h3>
            {/if}
            {#if sousTitre}
              <p class="carte-sous-titre">{sousTitre}</p>
            {/if}
          </div>
        </div>
      </slot>
    </div>
  {/if}
  
  <!-- Corps de la carte -->
  <div class="carte-body">
    <slot>
      <p class="carte-placeholder">Contenu de la carte</p>
    </slot>
  </div>
  
  <!-- Pied de page -->
  {#if $$slots.footer}
    <div class="carte-footer">
      <slot name="footer" />
    </div>
  {/if}
  
  <!-- Actions -->
  {#if $$slots.actions}
    <div class="carte-actions">
      <slot name="actions" />
    </div>
  {/if}
</div>

<style>
  .carte {
    /* Base de la carte */
    background: var(--couleur-fond-secondaire);
    border: 1px solid var(--couleur-bordure);
    border-radius: var(--rayon-bordure-lg);
    overflow: hidden;
    transition: all var(--transition-normale);
    position: relative;
    display: flex;
    flex-direction: column;
  }
  
  /* === Variants === */
  
  .carte-default {
    border-color: var(--couleur-bordure);
  }
  
  .carte-primary {
    border-color: var(--couleur-principale);
    background: linear-gradient(135deg, 
      var(--couleur-fond-secondaire) 0%, 
      rgba(99, 102, 241, 0.05) 100%);
  }
  
  .carte-success {
    border-color: var(--couleur-succes);
    background: linear-gradient(135deg, 
      var(--couleur-fond-secondaire) 0%, 
      rgba(16, 185, 129, 0.05) 100%);
  }
  
  .carte-warning {
    border-color: var(--couleur-avertissement);
    background: linear-gradient(135deg, 
      var(--couleur-fond-secondaire) 0%, 
      rgba(245, 158, 11, 0.05) 100%);
  }
  
  .carte-error {
    border-color: var(--couleur-erreur);
    background: linear-gradient(135deg, 
      var(--couleur-fond-secondaire) 0%, 
      rgba(239, 68, 68, 0.05) 100%);
  }
  
  /* === Élévations === */
  
  .carte-elevation-sm {
    box-shadow: var(--ombre-sm);
  }
  
  .carte-elevation-md {
    box-shadow: var(--ombre-md);
  }
  
  .carte-elevation-lg {
    box-shadow: var(--ombre-lg);
  }
  
  /* === États === */
  
  .carte-cliquable {
    cursor: pointer;
    user-select: none;
  }
  
  .carte-cliquable:hover {
    transform: translateY(-2px);
    box-shadow: var(--ombre-lg);
    border-color: var(--couleur-bordure-claire);
  }
  
  .carte-cliquable:focus {
    outline: 2px solid var(--couleur-bordure-focus);
    outline-offset: 2px;
  }
  
  .carte-cliquable:active {
    transform: translateY(0);
  }
  
  .carte-chargement {
    pointer-events: none;
  }
  
  /* === Overlay de chargement === */
  
  .carte-chargement-overlay {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(30, 41, 59, 0.8);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 10;
    backdrop-filter: blur(2px);
  }
  
  .carte-spinner {
    width: 32px;
    height: 32px;
    border: 3px solid rgba(99, 102, 241, 0.2);
    border-top: 3px solid var(--couleur-principale);
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }
  
  /* === Structure interne === */
  
  .carte-header {
    padding: var(--espacement-lg);
    border-bottom: 1px solid var(--couleur-bordure);
    background: var(--couleur-fond-tertiaire);
  }
  
  .carte-header-content {
    display: flex;
    align-items: flex-start;
    gap: var(--espacement-md);
  }
  
  .carte-icone {
    font-size: 1.5rem;
    line-height: 1;
    flex-shrink: 0;
  }
  
  .carte-header-text {
    flex: 1;
    min-width: 0; /* Permet la troncature */
  }
  
  .carte-titre {
    margin: 0 0 var(--espacement-xs) 0;
    font-size: var(--taille-police-lg);
    font-weight: 600;
    color: var(--couleur-texte-principal);
    line-height: var(--hauteur-ligne-serree);
  }
  
  .carte-sous-titre {
    margin: 0;
    font-size: var(--taille-police-sm);
    color: var(--couleur-texte-secondaire);
    line-height: var(--hauteur-ligne-normale);
  }
  
  .carte-body {
    padding: var(--espacement-lg);
    flex: 1;
    display: flex;
    flex-direction: column;
  }
  
  .carte-placeholder {
    margin: 0;
    color: var(--couleur-texte-tertiaire);
    font-style: italic;
  }
  
  .carte-footer {
    padding: var(--espacement-lg);
    border-top: 1px solid var(--couleur-bordure);
    background: var(--couleur-fond-tertiaire);
  }
  
  .carte-actions {
    padding: var(--espacement-lg);
    border-top: 1px solid var(--couleur-bordure);
    background: var(--couleur-fond-tertiaire);
    display: flex;
    gap: var(--espacement-sm);
    justify-content: flex-end;
    flex-wrap: wrap;
  }
  
  /* === Animations === */
  
  @keyframes spin {
    0% { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
  }
  
  /* === Responsive === */
  
  @media (max-width: 768px) {
    .carte-header,
    .carte-body,
    .carte-footer,
    .carte-actions {
      padding: var(--espacement-md);
    }
    
    .carte-header-content {
      gap: var(--espacement-sm);
    }
    
    .carte-icone {
      font-size: 1.25rem;
    }
    
    .carte-titre {
      font-size: var(--taille-police-base);
    }
    
    .carte-actions {
      justify-content: stretch;
    }
    
    .carte-actions :global(.btn) {
      flex: 1;
    }
  }
  
  /* === Accessibilité === */
  
  @media (prefers-reduced-motion: reduce) {
    .carte {
      transition: none;
    }
    
    .carte-cliquable:hover {
      transform: none;
    }
    
    .carte-spinner {
      animation: none;
    }
  }
  
  /* === Thème clair (si nécessaire) === */
  
  @media (prefers-color-scheme: light) {
    .carte-chargement-overlay {
      background: rgba(248, 250, 252, 0.8);
    }
  }
</style>