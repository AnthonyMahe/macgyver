<!--
  Composant IndicateurChargement - Indicateur de chargement réutilisable
  
  Props:
  - taille: 'sm' | 'md' | 'lg' | 'xl' - Taille de l'indicateur
  - couleur: 'primary' | 'secondary' | 'success' | 'warning' | 'error' - Couleur
  - message: string - Message à afficher
  - type: 'spinner' | 'dots' | 'pulse' | 'bars' - Type d'animation
  - centrer: boolean - Centrer l'indicateur
  - overlay: boolean - Afficher en overlay plein écran
  
  Slots:
  - default: Contenu personnalisé à afficher sous l'indicateur
-->

<script>
  // Props avec valeurs par défaut
  export let taille = 'md';
  export let couleur = 'primary';
  export let message = '';
  export let type = 'spinner';
  export let centrer = false;
  export let overlay = false;
  
  // Classes CSS calculées
  $: classes = [
    'indicateur',
    `indicateur-${taille}`,
    `indicateur-${couleur}`,
    `indicateur-${type}`,
    centrer ? 'indicateur-centre' : '',
    overlay ? 'indicateur-overlay' : ''
  ].filter(Boolean).join(' ');
  
  // Couleurs selon la variante
  $: couleurCSS = {
    primary: 'var(--couleur-principale)',
    secondary: 'var(--couleur-texte-secondaire)',
    success: 'var(--couleur-succes)',
    warning: 'var(--couleur-avertissement)',
    error: 'var(--couleur-erreur)'
  }[couleur] || 'var(--couleur-principale)';
</script>

<div class={classes} role="status" aria-live="polite">
  <!-- Overlay de fond si nécessaire -->
  {#if overlay}
    <div class="indicateur-fond"></div>
  {/if}
  
  <div class="indicateur-contenu">
    <!-- Indicateur visuel selon le type -->
    {#if type === 'spinner'}
      <div class="spinner" style="border-top-color: {couleurCSS};" aria-hidden="true"></div>
    {:else if type === 'dots'}
      <div class="dots" aria-hidden="true">
        <div class="dot" style="background-color: {couleurCSS};"></div>
        <div class="dot" style="background-color: {couleurCSS};"></div>
        <div class="dot" style="background-color: {couleurCSS};"></div>
      </div>
    {:else if type === 'pulse'}
      <div class="pulse" style="background-color: {couleurCSS};" aria-hidden="true"></div>
    {:else if type === 'bars'}
      <div class="bars" aria-hidden="true">
        <div class="bar" style="background-color: {couleurCSS};"></div>
        <div class="bar" style="background-color: {couleurCSS};"></div>
        <div class="bar" style="background-color: {couleurCSS};"></div>
        <div class="bar" style="background-color: {couleurCSS};"></div>
      </div>
    {/if}
    
    <!-- Message de chargement -->
    {#if message}
      <div class="indicateur-message">{message}</div>
    {/if}
    
    <!-- Contenu personnalisé -->
    {#if $$slots.default}
      <div class="indicateur-slot">
        <slot />
      </div>
    {/if}
  </div>
  
  <!-- Texte pour les lecteurs d'écran -->
  <span class="sr-only">
    {message || 'Chargement en cours...'}
  </span>
</div>

<style>
  .indicateur {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    flex-direction: column;
    gap: var(--espacement-md);
  }
  
  /* === Positionnement === */
  
  .indicateur-centre {
    position: fixed;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    z-index: var(--z-modal);
  }
  
  .indicateur-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    z-index: var(--z-modal);
    display: flex;
    align-items: center;
    justify-content: center;
  }
  
  .indicateur-fond {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(15, 23, 42, 0.8);
    backdrop-filter: blur(4px);
  }
  
  .indicateur-contenu {
    position: relative;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: var(--espacement-md);
    z-index: 1;
  }
  
  /* === Tailles === */
  
  .indicateur-sm {
    gap: var(--espacement-sm);
  }
  
  .indicateur-sm .spinner,
  .indicateur-sm .pulse {
    width: 20px;
    height: 20px;
  }
  
  .indicateur-sm .dots .dot {
    width: 6px;
    height: 6px;
  }
  
  .indicateur-sm .bars .bar {
    width: 3px;
    height: 16px;
  }
  
  .indicateur-md {
    gap: var(--espacement-md);
  }
  
  .indicateur-md .spinner,
  .indicateur-md .pulse {
    width: 32px;
    height: 32px;
  }
  
  .indicateur-md .dots .dot {
    width: 8px;
    height: 8px;
  }
  
  .indicateur-md .bars .bar {
    width: 4px;
    height: 24px;
  }
  
  .indicateur-lg {
    gap: var(--espacement-lg);
  }
  
  .indicateur-lg .spinner,
  .indicateur-lg .pulse {
    width: 48px;
    height: 48px;
  }
  
  .indicateur-lg .dots .dot {
    width: 12px;
    height: 12px;
  }
  
  .indicateur-lg .bars .bar {
    width: 6px;
    height: 32px;
  }
  
  .indicateur-xl {
    gap: var(--espacement-xl);
  }
  
  .indicateur-xl .spinner,
  .indicateur-xl .pulse {
    width: 64px;
    height: 64px;
  }
  
  .indicateur-xl .dots .dot {
    width: 16px;
    height: 16px;
  }
  
  .indicateur-xl .bars .bar {
    width: 8px;
    height: 40px;
  }
  
  /* === Types d'indicateurs === */
  
  /* Spinner rotatif */
  .spinner {
    border: 3px solid rgba(99, 102, 241, 0.1);
    border-top: 3px solid var(--couleur-principale);
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }
  
  /* Points qui bougent */
  .dots {
    display: flex;
    gap: var(--espacement-xs);
  }
  
  .dot {
    border-radius: 50%;
    animation: bounce 1.4s ease-in-out infinite both;
  }
  
  .dot:nth-child(1) { animation-delay: -0.32s; }
  .dot:nth-child(2) { animation-delay: -0.16s; }
  .dot:nth-child(3) { animation-delay: 0s; }
  
  /* Pulsation */
  .pulse {
    border-radius: 50%;
    animation: pulse 2s ease-in-out infinite;
  }
  
  /* Barres qui montent et descendent */
  .bars {
    display: flex;
    gap: var(--espacement-xs);
    align-items: flex-end;
  }
  
  .bar {
    border-radius: var(--rayon-bordure-sm);
    animation: bars 1.2s ease-in-out infinite;
  }
  
  .bar:nth-child(1) { animation-delay: -0.45s; }
  .bar:nth-child(2) { animation-delay: -0.3s; }
  .bar:nth-child(3) { animation-delay: -0.15s; }
  .bar:nth-child(4) { animation-delay: 0s; }
  
  /* === Message et contenu === */
  
  .indicateur-message {
    font-size: var(--taille-police-sm);
    color: var(--couleur-texte-secondaire);
    text-align: center;
    font-weight: 500;
  }
  
  .indicateur-slot {
    text-align: center;
  }
  
  /* === Animations === */
  
  @keyframes spin {
    0% { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
  }
  
  @keyframes bounce {
    0%, 80%, 100% {
      transform: scale(0);
      opacity: 0.5;
    }
    40% {
      transform: scale(1);
      opacity: 1;
    }
  }
  
  @keyframes pulse {
    0%, 100% {
      transform: scale(1);
      opacity: 1;
    }
    50% {
      transform: scale(1.1);
      opacity: 0.7;
    }
  }
  
  @keyframes bars {
    0%, 40%, 100% {
      transform: scaleY(0.4);
      opacity: 0.5;
    }
    20% {
      transform: scaleY(1);
      opacity: 1;
    }
  }
  
  /* === Responsive === */
  
  @media (max-width: 768px) {
    .indicateur-xl {
      gap: var(--espacement-lg);
    }
    
    .indicateur-xl .spinner,
    .indicateur-xl .pulse {
      width: 48px;
      height: 48px;
    }
    
    .indicateur-lg {
      gap: var(--espacement-md);
    }
    
    .indicateur-lg .spinner,
    .indicateur-lg .pulse {
      width: 32px;
      height: 32px;
    }
  }
  
  /* === Accessibilité === */
  
  @media (prefers-reduced-motion: reduce) {
    .spinner,
    .dot,
    .pulse,
    .bar {
      animation: none;
    }
    
    /* Indicateur statique pour les utilisateurs qui préfèrent moins d'animations */
    .spinner {
      border-top-color: var(--couleur-principale);
      opacity: 0.8;
    }
    
    .dot {
      opacity: 0.6;
    }
    
    .pulse {
      opacity: 0.8;
    }
    
    .bar {
      opacity: 0.6;
    }
  }
  
  /* === Thème clair === */
  
  @media (prefers-color-scheme: light) {
    .indicateur-fond {
      background: rgba(248, 250, 252, 0.9);
    }
  }
</style>