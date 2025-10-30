<!--
  Page TodoList Simplifiée
  
  Interface simple de gestion de tâches avec :
  - Ajout rapide de tâches
  - Marquage terminé/non terminé
  - Suppression simple
-->

<script>
  import { onMount } from 'svelte';
  import { BoutonPrincipal } from '$lib/composants';
  
  // État des tâches
  let taches = [];
  let nouvelleTache = '';
  
  // Fonctions de gestion des tâches
  function ajouterTache() {
    if (nouvelleTache.trim()) {
      const tache = {
        id: Date.now().toString(),
        titre: nouvelleTache.trim(),
        terminee: false,
        dateCreation: new Date()
      };
      
      taches = [tache, ...taches];
      nouvelleTache = '';
      sauvegarderTaches();
    }
  }
  
  function basculerTache(id) {
    taches = taches.map(tache => 
      tache.id === id 
        ? { ...tache, terminee: !tache.terminee }
        : tache
    );
    sauvegarderTaches();
  }
  
  function supprimerTache(id) {
    taches = taches.filter(tache => tache.id !== id);
    sauvegarderTaches();
  }
  
  function sauvegarderTaches() {
    localStorage.setItem('productivapp_taches', JSON.stringify(taches));
  }
  
  function chargerTaches() {
    const tachesSauvees = localStorage.getItem('productivapp_taches');
    if (tachesSauvees) {
      taches = JSON.parse(tachesSauvees);
    }
  }
  
  // Gestion des raccourcis clavier
  function gererTouche(event) {
    if (event.key === 'Enter') {
      ajouterTache();
    }
  }
  
  // Statistiques simples
  $: tachesTerminees = taches.filter(t => t.terminee).length;
  $: tachesEnCours = taches.filter(t => !t.terminee).length;
  
  onMount(() => {
    chargerTaches();
  });
</script>

<svelte:head>
  <title>TodoList - MacGyver</title>
</svelte:head>

<div class="todos-container">
  
  <!-- En-tête -->
  <div class="header">
    <h1>📝 Ma TodoList</h1>
    <div class="stats">
      <span class="stat">
        <span class="stat-number">{tachesEnCours}</span>
        <span class="stat-label">à faire</span>
      </span>
      <span class="stat">
        <span class="stat-number">{tachesTerminees}</span>
        <span class="stat-label">terminées</span>
      </span>
    </div>
  </div>
  
  <!-- Ajout de tâche -->
  <div class="add-section">
    <div class="add-input">
      <input 
        type="text" 
        bind:value={nouvelleTache}
        on:keydown={gererTouche}
        placeholder="Ajouter une nouvelle tâche..."
        class="input-tache"
      />
      <BoutonPrincipal 
        texte="Ajouter"
        variante="primary"
        taille="md"
        desactive={!nouvelleTache.trim()}
        on:click={ajouterTache}
      />
    </div>
  </div>
  
  <!-- Liste des tâches -->
  <div class="taches-list">
    {#if taches.length === 0}
      <div class="empty-state">
        <p>🎯 Aucune tâche pour le moment</p>
        <p>Ajoutez votre première tâche ci-dessus !</p>
      </div>
    {:else}
      {#each taches as tache (tache.id)}
        <div class="tache-item" class:terminee={tache.terminee}>
          <label class="tache-checkbox">
            <input 
              type="checkbox" 
              checked={tache.terminee}
              on:change={() => basculerTache(tache.id)}
            />
            <span class="checkmark"></span>
          </label>
          
          <span class="tache-titre" class:barre={tache.terminee}>
            {tache.titre}
          </span>
          
          <button 
            class="btn-delete"
            on:click={() => supprimerTache(tache.id)}
            title="Supprimer"
          >
            🗑️
          </button>
        </div>
      {/each}
    {/if}
  </div>
  
</div>

<style>
  .todos-container {
    max-width: 800px;
    margin: 0 auto;
    padding: var(--espacement-xl);
    display: flex;
    flex-direction: column;
    gap: var(--espacement-xl);
    height: 100%;
  }
  
  /* En-tête */
  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding-bottom: var(--espacement-lg);
    border-bottom: 2px solid var(--couleur-bordure);
  }
  
  .header h1 {
    margin: 0;
    color: var(--couleur-texte-principal);
    font-size: 2rem;
  }
  
  .stats {
    display: flex;
    gap: var(--espacement-lg);
  }
  
  .stat {
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: var(--espacement-md);
    background: var(--couleur-fond-secondaire);
    border-radius: var(--rayon-bordure-md);
    min-width: 80px;
  }
  
  .stat-number {
    font-size: 1.5rem;
    font-weight: 700;
    color: var(--couleur-principale);
  }
  
  .stat-label {
    font-size: 0.85rem;
    color: var(--couleur-texte-secondaire);
  }
  
  /* Section d'ajout */
  .add-section {
    background: var(--couleur-fond-secondaire);
    border: 1px solid var(--couleur-bordure);
    padding: var(--espacement-xl);
    border-radius: var(--rayon-bordure-lg);
    box-shadow: var(--ombre-md);
  }
  
  .add-input {
    display: flex;
    gap: var(--espacement-md);
  }
  
  .input-tache {
    flex: 1;
    padding: var(--espacement-md);
    background: var(--couleur-fond-tertiaire);
    border: 1px solid var(--couleur-bordure);
    border-radius: var(--rayon-bordure-md);
    color: var(--couleur-texte-principal);
    font-size: 1rem;
    transition: all var(--transition-rapide);
  }
  
  .input-tache:focus {
    outline: none;
    border-color: var(--couleur-principale);
    box-shadow: 0 0 0 3px rgba(99, 102, 241, 0.15);
    background: var(--couleur-fond-secondaire);
    transform: translateY(-1px);
  }
  
  .input-tache::placeholder {
    color: var(--couleur-texte-tertiaire);
  }
  
  .btn-add {
    padding: var(--espacement-md) var(--espacement-xl);
    background: var(--couleur-principale);
    color: white;
    border: none;
    border-radius: var(--rayon-bordure-md);
    font-weight: 600;
    cursor: pointer;
    transition: all var(--transition-rapide);
  }
  
  .btn-add:hover:not(:disabled) {
    background: var(--couleur-principale-foncee);
    transform: translateY(-1px);
  }
  
  .btn-add:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
  
  /* Liste des tâches */
  .taches-list {
    flex: 1;
    background: var(--couleur-fond-secondaire);
    border: 1px solid var(--couleur-bordure);
    border-radius: var(--rayon-bordure-lg);
    box-shadow: var(--ombre-md);
    overflow: hidden;
  }
  
  .tache-item {
    display: flex;
    align-items: center;
    gap: var(--espacement-md);
    padding: var(--espacement-lg);
    border-bottom: 1px solid var(--couleur-bordure);
    transition: all var(--transition-rapide);
  }
  
  .tache-item:hover {
    background: var(--couleur-fond-tertiaire);
  }
  
  .tache-item:last-child {
    border-bottom: none;
  }
  
  .tache-item.terminee {
    opacity: 0.6;
  }
  
  /* Checkbox personnalisée */
  .tache-checkbox {
    position: relative;
    cursor: pointer;
    display: flex;
    align-items: center;
  }
  
  .tache-checkbox input {
    opacity: 0;
    position: absolute;
  }
  
  .checkmark {
    width: 20px;
    height: 20px;
    border: 2px solid var(--couleur-principale);
    border-radius: 4px;
    display: block;
    position: relative;
    transition: all var(--transition-rapide);
  }
  
  .tache-checkbox input:checked + .checkmark {
    background: var(--couleur-principale);
  }
  
  .tache-checkbox input:checked + .checkmark::after {
    content: '✓';
    color: white;
    position: absolute;
    top: -2px;
    left: 3px;
    font-size: 14px;
    font-weight: bold;
  }
  
  /* Titre de tâche */
  .tache-titre {
    flex: 1;
    font-size: 1rem;
    color: var(--couleur-texte-principal);
    transition: all var(--transition-rapide);
  }
  
  .tache-titre.barre {
    text-decoration: line-through;
    color: var(--couleur-texte-secondaire);
  }
  
  /* Bouton de suppression */
  .btn-delete {
    background: none;
    border: none;
    font-size: 1.2rem;
    cursor: pointer;
    opacity: 0.6;
    transition: opacity var(--transition-rapide);
    padding: var(--espacement-xs);
    border-radius: var(--rayon-bordure-sm);
  }
  
  .btn-delete:hover {
    opacity: 1;
    background: rgba(231, 76, 60, 0.1);
  }
  
  /* État vide */
  .empty-state {
    text-align: center;
    padding: var(--espacement-2xl);
    color: var(--couleur-texte-secondaire);
  }
  
  .empty-state p:first-child {
    font-size: 1.2rem;
    margin-bottom: var(--espacement-sm);
  }
  
  /* Responsive */
  @media (max-width: 768px) {
    .todos-container {
      padding: var(--espacement-lg);
    }
    
    .header {
      flex-direction: column;
      gap: var(--espacement-md);
      text-align: center;
    }
    
    .add-input {
      flex-direction: column;
    }
    
    .tache-item {
      padding: var(--espacement-md);
    }
  }
</style>