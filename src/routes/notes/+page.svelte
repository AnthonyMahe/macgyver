<!--
  Page Notes Rapides
  
  Interface de prise de notes avec :
  - Création/édition rapide
  - Recherche et filtrage
  - Catégories et tags
  - Sauvegarde automatique
-->

<script>
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/tauri';
  import { BoutonPrincipal } from '$lib/composants';
  
  // État des notes
  let notes = [];
  let noteActive = null;
  let modeEdition = false;
  
  // Filtres et recherche
  let rechercheTexte = '';
  let categorieFiltre = 'toutes';
  let categories = ['Personnel', 'Travail', 'Idées', 'Projets', 'Urgent'];
  
  // Nouvelle note
  let nouvelleNote = {
    titre: '',
    contenu: '',
    categorie: 'Personnel',
    tags: [],
    dateCreation: null,
    dateModification: null
  };
  
  // Interface
  let affichageMode = 'grille'; // 'grille' ou 'liste'
  let panneauOuvert = true;
  
  // Couleurs des catégories
  function getCategorieColor(categorie) {
    const couleurs = {
      'Personnel': '#3498db',
      'Travail': '#e74c3c',
      'Idées': '#f39c12',
      'Projets': '#27ae60',
      'Urgent': '#9b59b6'
    };
    return couleurs[categorie] || '#95a5a6';
  }
  
  // Fonction pour fermer le panneau de détail
  function fermerPanneau() {
    noteActive = null;
    modeEdition = false;
  }
  
  // Gestion des clics en dehors du panneau
  function gererClicExterieur(event) {
    if (noteActive && !event.target.closest('.note-detail') && !event.target.closest('.note-card')) {
      fermerPanneau();
    }
  }
  
  // Fonctions de gestion des notes
  function creerNouvelleNote() {
    const note = {
      id: Date.now().toString(),
      titre: nouvelleNote.titre || 'Note sans titre',
      contenu: nouvelleNote.contenu,
      categorie: nouvelleNote.categorie,
      tags: nouvelleNote.tags,
      dateCreation: new Date(),
      dateModification: new Date(),
      epinglee: false
    };
    
    notes = [note, ...notes];
    sauvegarderNotes();
    
    // Réinitialiser le formulaire
    nouvelleNote = {
      titre: '',
      contenu: '',
      categorie: 'Personnel',
      tags: [],
      dateCreation: null,
      dateModification: null
    };
    
    // Sélectionner la nouvelle note
    selectionnerNote(note);
  }
  
  function selectionnerNote(note) {
    noteActive = note;
    modeEdition = false;
  }
  
  function modifierNote() {
    modeEdition = true;
  }
  
  function sauvegarderModification() {
    if (noteActive) {
      noteActive.dateModification = new Date();
      notes = [...notes]; // Trigger reactivity
      sauvegarderNotes();
      modeEdition = false;
    }
  }
  
  function annulerModification() {
    modeEdition = false;
    // Recharger la note depuis le stockage
    chargerNotes();
  }
  
  function supprimerNote(note) {
    if (confirm(`Êtes-vous sûr de vouloir supprimer "${note.titre}" ?`)) {
      notes = notes.filter(n => n.id !== note.id);
      if (noteActive && noteActive.id === note.id) {
        noteActive = null;
      }
      sauvegarderNotes();
    }
  }
  
  function epinglerNote(note) {
    note.epinglee = !note.epinglee;
    // Réorganiser : notes épinglées en premier
    notes = notes.sort((a, b) => {
      if (a.epinglee && !b.epinglee) return -1;
      if (!a.epinglee && b.epinglee) return 1;
      return new Date(b.dateModification) - new Date(a.dateModification);
    });
    sauvegarderNotes();
  }
  
  function ajouterTag(note, tag) {
    if (tag && !note.tags.includes(tag)) {
      note.tags = [...note.tags, tag];
      sauvegarderNotes();
    }
  }
  
  function supprimerTag(note, tag) {
    note.tags = note.tags.filter(t => t !== tag);
    sauvegarderNotes();
  }
  
  // Filtrage et recherche
  $: notesFiltrees = notes.filter(note => {
    const correspondRecherche = rechercheTexte === '' || 
      note.titre.toLowerCase().includes(rechercheTexte.toLowerCase()) ||
      note.contenu.toLowerCase().includes(rechercheTexte.toLowerCase()) ||
      note.tags.some(tag => tag.toLowerCase().includes(rechercheTexte.toLowerCase()));
    
    const correspondCategorie = categorieFiltre === 'toutes' || note.categorie === categorieFiltre;
    
    return correspondRecherche && correspondCategorie;
  });
  
  // Sauvegarde et chargement
  function sauvegarderNotes() {
    localStorage.setItem('macgyver_notes', JSON.stringify(notes));
  }
  
  function chargerNotes() {
    const notesString = localStorage.getItem('macgyver_notes');
    if (notesString) {
      notes = JSON.parse(notesString);
    } else {
      // Notes d'exemple
      notes = [
        {
          id: '1',
          titre: 'Bienvenue dans vos notes !',
          contenu: 'Ceci est votre première note. Vous pouvez :\n\n• Créer de nouvelles notes\n• Les organiser par catégories\n• Ajouter des tags\n• Rechercher dans vos notes\n• Épingler les importantes\n\nCommencez dès maintenant !',
          categorie: 'Personnel',
          tags: ['bienvenue', 'guide'],
          dateCreation: new Date(),
          dateModification: new Date(),
          epinglee: true
        }
      ];
    }
  }
  
  // Formatage des dates
  function formaterDate(date) {
    return new Date(date).toLocaleDateString('fr-FR', {
      day: 'numeric',
      month: 'short',
      hour: '2-digit',
      minute: '2-digit'
    });
  }
  
  // Raccourcis clavier
  function gererRaccourcis(event) {
    if (event.ctrlKey || event.metaKey) {
      if (event.key === 'n') {
        event.preventDefault();
        document.getElementById('nouveau-titre').focus();
      } else if (event.key === 's' && modeEdition) {
        event.preventDefault();
        sauvegarderModification();
      } else if (event.key === 'f') {
        event.preventDefault();
        document.getElementById('recherche').focus();
      }
    }
    
    if (event.key === 'Escape') {
      if (modeEdition) {
        annulerModification();
      } else if (noteActive) {
        fermerPanneau();
      }
    }
  }
  
  onMount(() => {
    chargerNotes();
    window.addEventListener('keydown', gererRaccourcis);
    window.addEventListener('click', gererClicExterieur);
    
    return () => {
      window.removeEventListener('keydown', gererRaccourcis);
      window.removeEventListener('click', gererClicExterieur);
    };
  });
</script>

<svelte:head>
  <title>Notes Rapides - MacGyver</title>
</svelte:head>

<div class="notes-container">
  
  <!-- Panneau latéral -->
  <div class="sidebar-notes" class:sidebar-ferme={!panneauOuvert}>
    
    <!-- Nouvelle note -->
    <div class="nouvelle-note-section">
      <h3>📝 Nouvelle note</h3>
      
      <div class="form-group">
        <input 
          id="nouveau-titre"
          type="text" 
          placeholder="Titre de la note..."
          bind:value={nouvelleNote.titre}
          class="input-titre"
        />
      </div>
      
      <div class="form-group">
        <select bind:value={nouvelleNote.categorie} class="select-categorie">
          {#each categories as categorie}
            <option value={categorie}>{categorie}</option>
          {/each}
        </select>
      </div>
      
      <div class="form-group">
        <textarea 
          placeholder="Contenu de la note..."
          bind:value={nouvelleNote.contenu}
          class="textarea-contenu"
          rows="4"
        ></textarea>
      </div>
      
      <BoutonPrincipal 
        texte="Créer la note"
        icone="➕"
        variante="primary"
        taille="md"
        largeurComplete={true}
        desactive={!nouvelleNote.contenu.trim()}
        on:click={creerNouvelleNote}
      />
    </div>
    
    <!-- Recherche et filtres -->
    <div class="recherche-section">
      <h3>🔍 Recherche</h3>
      
      <input 
        id="recherche"
        type="text" 
        placeholder="Rechercher dans les notes..."
        bind:value={rechercheTexte}
        class="input-recherche"
      />
      
      <select bind:value={categorieFiltre} class="select-filtre">
        <option value="toutes">Toutes les catégories</option>
        {#each categories as categorie}
          <option value={categorie}>{categorie}</option>
        {/each}
      </select>
    </div>
    
    <!-- Statistiques -->
    <div class="stats-notes">
      <h3>📊 Statistiques</h3>
      <div class="stat-item">
        <span class="stat-number">{notes.length}</span>
        <span class="stat-label">Notes totales</span>
      </div>
      <div class="stat-item">
        <span class="stat-number">{notesFiltrees.length}</span>
        <span class="stat-label">Notes affichées</span>
      </div>
      <div class="stat-item">
        <span class="stat-number">{notes.filter(n => n.epinglee).length}</span>
        <span class="stat-label">Notes épinglées</span>
      </div>
    </div>
    
    <!-- Raccourcis -->
    <div class="raccourcis-section">
      <h4>⌨️ Raccourcis</h4>
      <div class="raccourci-item">
        <kbd>Ctrl+N</kbd> Nouvelle note
      </div>
      <div class="raccourci-item">
        <kbd>Ctrl+F</kbd> Rechercher
      </div>
      <div class="raccourci-item">
        <kbd>Ctrl+S</kbd> Sauvegarder
      </div>
      <div class="raccourci-item">
        <kbd>Échap</kbd> Annuler
      </div>
    </div>
    
  </div>
  
  <!-- Zone principale -->
  <div class="main-notes">
    
    <!-- Barre d'outils -->
    <div class="toolbar-notes">
      <div class="toolbar-left">
        <button 
          class="btn-toggle-sidebar"
          on:click={() => panneauOuvert = !panneauOuvert}
        >
          {panneauOuvert ? '◀' : '▶'}
        </button>
        
        <div class="view-controls">
          <button 
            class="btn-view"
            class:active={affichageMode === 'grille'}
            on:click={() => affichageMode = 'grille'}
          >
            ⊞ Grille
          </button>
          <button 
            class="btn-view"
            class:active={affichageMode === 'liste'}
            on:click={() => affichageMode = 'liste'}
          >
            ☰ Liste
          </button>
        </div>
      </div>
      
      <div class="toolbar-right">
        <span class="notes-count">
          {notesFiltrees.length} note{notesFiltrees.length > 1 ? 's' : ''}
        </span>
      </div>
    </div>
    
    <!-- Liste des notes -->
    <div class="notes-content">
      
      {#if notesFiltrees.length === 0}
        <div class="empty-state">
          <div class="empty-icon">📝</div>
          <h3>Aucune note trouvée</h3>
          <p>
            {#if rechercheTexte || categorieFiltre !== 'toutes'}
              Essayez de modifier vos critères de recherche.
            {:else}
              Créez votre première note pour commencer !
            {/if}
          </p>
        </div>
      {:else}
        <div class="notes-grid" class:notes-liste={affichageMode === 'liste'}>
          {#each notesFiltrees as note (note.id)}
            <div 
              class="note-card"
              class:note-active={noteActive && noteActive.id === note.id}
              class:note-epinglee={note.epinglee}
              role="button"
              tabindex="0"
              on:click={() => selectionnerNote(note)}
              on:keydown={(e) => e.key === 'Enter' && selectionnerNote(note)}
            >
              
              <!-- En-tête de la note -->
              <div class="note-header">
                <div class="note-title-section">
                  {#if note.epinglee}
                    <span class="pin-icon">📌</span>
                  {/if}
                  <h4 class="note-title">{note.titre}</h4>
                </div>
                
                <div class="note-actions">
                  <button 
                    class="btn-action"
                    on:click|stopPropagation={() => epinglerNote(note)}
                    title={note.epinglee ? 'Désépingler' : 'Épingler'}
                  >
                    {note.epinglee ? '📌' : '📍'}
                  </button>
                  <button 
                    class="btn-action"
                    on:click|stopPropagation={() => supprimerNote(note)}
                    title="Supprimer"
                  >
                    🗑️
                  </button>
                </div>
              </div>
              
              <!-- Contenu de la note -->
              <div class="note-content-preview">
                {note.contenu.substring(0, 150)}{note.contenu.length > 150 ? '...' : ''}
              </div>
              
              <!-- Métadonnées -->
              <div class="note-meta">
                <span class="note-categorie" style="background-color: {getCategorieColor(note.categorie)}">
                  {note.categorie}
                </span>
                
                {#if note.tags.length > 0}
                  <div class="note-tags">
                    {#each note.tags.slice(0, 3) as tag}
                      <span class="tag">#{tag}</span>
                    {/each}
                    {#if note.tags.length > 3}
                      <span class="tag-more">+{note.tags.length - 3}</span>
                    {/if}
                  </div>
                {/if}
                
                <div class="note-date">
                  {formaterDate(note.dateModification)}
                </div>
              </div>
              
            </div>
          {/each}
        </div>
      {/if}
      
    </div>
    
  </div>
  
  <!-- Panneau de détail -->
  {#if noteActive}
    <div class="detail-panel">
      
      <!-- En-tête du détail -->
      <div class="detail-header">
        <div class="detail-title-section">
          {#if modeEdition}
            <input 
              type="text" 
              bind:value={noteActive.titre}
              class="input-titre-edit"
            />
          {:else}
            <h2 class="detail-title">{noteActive.titre}</h2>
          {/if}
        </div>
        
        <div class="detail-actions">
          <button 
            class="btn-icon btn-close" 
            on:click={fermerPanneau}
            title="Fermer (Échap)"
          >
            ✕
          </button>
          {#if modeEdition}
            <button class="btn-action btn-save" on:click={sauvegarderModification}>
              💾 Sauvegarder
            </button>
            <button class="btn-action btn-cancel" on:click={annulerModification}>
              ❌ Annuler
            </button>
          {:else}
            <button class="btn-action" on:click={modifierNote}>
              ✏️ Modifier
            </button>
            <button class="btn-action" on:click={() => epinglerNote(noteActive)}>
              {noteActive.epinglee ? '📌' : '📍'}
            </button>
            <button class="btn-action" on:click={() => supprimerNote(noteActive)}>
              🗑️
            </button>
          {/if}
        </div>
      </div>
      
      <!-- Métadonnées détaillées -->
      <div class="detail-meta">
        <div class="meta-item">
          <span class="meta-label">Catégorie :</span>
          {#if modeEdition}
            <select bind:value={noteActive.categorie} class="select-categorie-edit">
              {#each categories as categorie}
                <option value={categorie}>{categorie}</option>
              {/each}
            </select>
          {:else}
            <span class="meta-value categorie-badge" style="background-color: {getCategorieColor(noteActive.categorie)}">
              {noteActive.categorie}
            </span>
          {/if}
        </div>
        
        <div class="meta-item">
          <span class="meta-label">Créée :</span>
          <span class="meta-value">{formaterDate(noteActive.dateCreation)}</span>
        </div>
        
        <div class="meta-item">
          <span class="meta-label">Modifiée :</span>
          <span class="meta-value">{formaterDate(noteActive.dateModification)}</span>
        </div>
      </div>
      
      <!-- Contenu -->
      <div class="detail-content">
        {#if modeEdition}
          <textarea 
            bind:value={noteActive.contenu}
            class="textarea-edit"
            placeholder="Contenu de la note..."
          ></textarea>
        {:else}
          <div class="content-display">
            {noteActive.contenu}
          </div>
        {/if}
      </div>
      
      <!-- Tags -->
      <div class="detail-tags">
        <h4>🏷️ Tags</h4>
        <div class="tags-container">
          {#each noteActive.tags as tag}
            <span class="tag-item">
              #{tag}
              {#if modeEdition}
                <button 
                  class="btn-remove-tag"
                  on:click={() => supprimerTag(noteActive, tag)}
                >
                  ×
                </button>
              {/if}
            </span>
          {/each}
          
          {#if modeEdition}
            <input 
              type="text" 
              placeholder="Nouveau tag..."
              class="input-nouveau-tag"
              on:keydown={(e) => {
                if (e.key === 'Enter' && e.target.value.trim()) {
                  ajouterTag(noteActive, e.target.value.trim());
                  e.target.value = '';
                }
              }}
            />
          {/if}
        </div>
      </div>
      
    </div>
  {/if}
  
</div>



<style>
  /* === Layout principal === */
  .notes-container {
    display: grid;
    grid-template-columns: 350px 1fr auto;
    height: 100%;
    gap: 0;
    background: var(--couleur-fond-secondaire);
  }
  
  /* === Sidebar === */
  .sidebar-notes {
    background: var(--couleur-fond-secondaire);
    border-right: 1px solid var(--couleur-bordure);
    display: flex;
    flex-direction: column;
    gap: var(--espacement-xl);
    padding: var(--espacement-xl);
    overflow-y: auto;
    transition: transform var(--transition-normale);
  }
  
  .sidebar-ferme {
    transform: translateX(-100%);
  }
  
  .sidebar-notes h3 {
    margin: 0 0 var(--espacement-md) 0;
    font-size: 1.1rem;
    color: var(--couleur-texte-principal);
  }
  
  /* === Nouvelle note === */
  .nouvelle-note-section {
    border-bottom: 1px solid var(--couleur-bordure);
    padding-bottom: var(--espacement-xl);
  }
  
  .form-group {
    margin-bottom: var(--espacement-md);
  }
  
  .input-titre,
  .select-categorie,
  .textarea-contenu,
  .input-recherche,
  .select-filtre {
    width: 100%;
    padding: var(--espacement-sm);
    background: var(--couleur-fond-tertiaire);
    border: 1px solid var(--couleur-bordure);
    border-radius: var(--rayon-bordure-sm);
    color: var(--couleur-texte-principal);
    font-family: inherit;
    transition: all var(--transition-rapide);
  }
  
  .input-titre:focus,
  .select-categorie:focus,
  .textarea-contenu:focus,
  .input-recherche:focus,
  .select-filtre:focus {
    outline: none;
    border-color: var(--couleur-principale);
    box-shadow: 0 0 0 3px rgba(99, 102, 241, 0.15);
    background: var(--couleur-fond-secondaire);
    transform: translateY(-1px);
  }
  
  .input-titre::placeholder,
  .textarea-contenu::placeholder,
  .input-recherche::placeholder {
    color: var(--couleur-texte-tertiaire);
  }
  
  .textarea-contenu {
    resize: vertical;
    min-height: 80px;
  }
  
  .btn-creer-note {
    width: 100%;
    padding: var(--espacement-md);
    background: var(--couleur-principale);
    color: white;
    border: none;
    border-radius: var(--rayon-bordure-md);
    cursor: pointer;
    font-weight: 500;
    transition: background-color var(--transition-rapide);
  }
  
  .btn-creer-note:hover:not(:disabled) {
    background: var(--couleur-principale-foncee);
  }
  
  .btn-creer-note:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
  
  /* === Recherche === */
  .recherche-section {
    border-bottom: 1px solid var(--couleur-bordure);
    padding-bottom: var(--espacement-xl);
  }
  
  /* === Statistiques === */
  .stats-notes {
    border-bottom: 1px solid var(--couleur-bordure);
    padding-bottom: var(--espacement-xl);
  }
  
  .stat-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: var(--espacement-sm);
  }
  
  .stat-number {
    font-weight: 700;
    color: var(--couleur-principale);
  }
  
  .stat-label {
    font-size: 0.9rem;
    color: var(--couleur-texte-secondaire);
  }
  
  /* === Raccourcis === */
  .raccourcis-section h4 {
    margin: 0 0 var(--espacement-md) 0;
    font-size: 0.95rem;
    color: var(--couleur-texte-principal);
  }
  
  .raccourci-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: var(--espacement-xs);
    font-size: 0.85rem;
  }
  
  kbd {
    background: var(--couleur-fond-secondaire);
    border: 1px solid var(--couleur-bordure);
    border-radius: 3px;
    padding: 2px 6px;
    font-family: var(--police-monospace);
    font-size: 0.75rem;
  }
  
  /* === Zone principale === */
  .main-notes {
    display: flex;
    flex-direction: column;
    background: var(--couleur-fond-principal);
  }
  
  /* === Toolbar === */
  .toolbar-notes {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: var(--espacement-md) var(--espacement-lg);
    border-bottom: 1px solid var(--couleur-bordure);
    background: var(--couleur-fond-secondaire);
  }
  
  .toolbar-left {
    display: flex;
    align-items: center;
    gap: var(--espacement-md);
  }
  
  .btn-toggle-sidebar {
    background: var(--couleur-fond-tertiaire);
    border: 1px solid var(--couleur-bordure);
    color: var(--couleur-texte-principal);
    padding: var(--espacement-xs) var(--espacement-sm);
    border-radius: var(--rayon-bordure-sm);
    cursor: pointer;
    transition: all var(--transition-rapide);
  }
  
  .btn-toggle-sidebar:hover {
    background: var(--couleur-fond-quaternaire);
    border-color: var(--couleur-bordure-claire);
  }
  
  .view-controls {
    display: flex;
    gap: var(--espacement-xs);
  }
  
  .btn-view {
    padding: var(--espacement-xs) var(--espacement-sm);
    border: 1px solid var(--couleur-bordure);
    background: var(--couleur-fond-tertiaire);
    color: var(--couleur-texte-principal);
    cursor: pointer;
    font-size: 0.85rem;
    transition: all var(--transition-rapide);
  }
  
  .btn-view:hover:not(.active) {
    background: var(--couleur-fond-quaternaire);
    border-color: var(--couleur-bordure-claire);
  }
  
  .btn-view.active {
    background: var(--couleur-principale);
    color: white;
    border-color: var(--couleur-principale);
  }
  
  .notes-count {
    font-size: 0.9rem;
    color: var(--couleur-texte-secondaire);
  }
  
  /* === Contenu des notes === */
  .notes-content {
    flex: 1;
    padding: var(--espacement-lg);
    overflow-y: auto;
  }
  
  .empty-state {
    text-align: center;
    padding: var(--espacement-3xl);
    color: var(--couleur-texte-secondaire);
  }
  
  .empty-icon {
    font-size: 4rem;
    margin-bottom: var(--espacement-lg);
  }
  
  /* === Grille des notes === */
  .notes-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
    gap: var(--espacement-lg);
  }
  
  .notes-liste {
    grid-template-columns: 1fr;
  }
  
  .note-card {
    background: var(--couleur-fond-secondaire);
    border: 1px solid var(--couleur-bordure);
    border-radius: var(--rayon-bordure-md);
    padding: var(--espacement-lg);
    cursor: pointer;
    transition: all var(--transition-rapide);
  }
  
  .note-card:hover {
    transform: translateY(-2px);
    box-shadow: var(--ombre-md);
  }
  
  .note-active {
    border-color: var(--couleur-principale);
    box-shadow: var(--ombre-md);
  }
  
  .note-epinglee {
    border-left: 4px solid #f39c12;
  }
  
  /* === En-tête de note === */
  .note-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    margin-bottom: var(--espacement-md);
  }
  
  .note-title-section {
    display: flex;
    align-items: center;
    gap: var(--espacement-xs);
    flex: 1;
  }
  
  .pin-icon {
    color: #f39c12;
  }
  
  .note-title {
    margin: 0;
    font-size: 1.1rem;
    color: var(--couleur-texte-principal);
    line-height: 1.3;
  }
  
  .note-actions {
    display: flex;
    gap: var(--espacement-xs);
    opacity: 0;
    transition: opacity var(--transition-rapide);
  }
  
  .note-card:hover .note-actions {
    opacity: 1;
  }
  
  .btn-action {
    background: none;
    border: none;
    cursor: pointer;
    padding: var(--espacement-xs);
    border-radius: var(--rayon-bordure-sm);
    transition: background-color var(--transition-rapide);
  }
  
  .btn-action:hover {
    background: var(--couleur-fond-secondaire);
  }
  
  /* === Contenu et métadonnées === */
  .note-content-preview {
    margin-bottom: var(--espacement-md);
    color: var(--couleur-texte-secondaire);
    line-height: 1.4;
    font-size: 0.9rem;
  }
  
  .note-meta {
    display: flex;
    flex-wrap: wrap;
    gap: var(--espacement-sm);
    align-items: center;
    font-size: 0.8rem;
  }
  
  .note-categorie {
    padding: var(--espacement-xs) var(--espacement-sm);
    border-radius: var(--rayon-bordure-xl);
    color: white;
    font-weight: 500;
  }
  
  .note-tags {
    display: flex;
    gap: var(--espacement-xs);
  }
  
  .tag {
    color: var(--couleur-principale);
    font-size: 0.75rem;
  }
  
  .tag-more {
    color: var(--couleur-texte-secondaire);
    font-size: 0.75rem;
  }
  
  .note-date {
    color: var(--couleur-texte-secondaire);
    margin-left: auto;
  }
  
  /* === Panneau de détail === */
  .detail-panel {
    width: 400px;
    background: var(--couleur-fond-secondaire);
    border-left: 1px solid var(--couleur-bordure);
    display: flex;
    flex-direction: column;
    overflow-y: auto;
  }
  
  .detail-header {
    padding: var(--espacement-lg);
    border-bottom: 1px solid var(--couleur-bordure);
    background: var(--couleur-fond-secondaire);
  }
  
  .detail-title-section {
    margin-bottom: var(--espacement-md);
  }
  
  .detail-title {
    margin: 0;
    font-size: 1.3rem;
    color: var(--couleur-texte-principal);
  }
  
  .input-titre-edit {
    width: 100%;
    font-size: 1.3rem;
    font-weight: 600;
    border: 1px solid var(--couleur-bordure);
    padding: var(--espacement-sm);
    border-radius: var(--rayon-bordure-sm);
  }
  
  .detail-actions {
    display: flex;
    gap: var(--espacement-sm);
  }
  
  .btn-save {
    background: var(--couleur-succes);
    color: white;
    padding: var(--espacement-xs) var(--espacement-md);
    border-radius: var(--rayon-bordure-sm);
  }
  
  .btn-cancel {
    background: var(--couleur-erreur);
    color: white;
    padding: var(--espacement-xs) var(--espacement-md);
    border-radius: var(--rayon-bordure-sm);
  }
  
  /* === Métadonnées détaillées === */
  .detail-meta {
    padding: var(--espacement-lg);
    border-bottom: 1px solid var(--couleur-bordure);
  }
  
  .meta-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: var(--espacement-sm);
  }
  
  .meta-label {
    font-weight: 500;
    color: var(--couleur-texte-principal);
  }
  
  .meta-value {
    color: var(--couleur-texte-secondaire);
  }
  
  .categorie-badge {
    padding: var(--espacement-xs) var(--espacement-sm);
    border-radius: var(--rayon-bordure-xl);
    color: white;
    font-weight: 500;
  }
  
  .select-categorie-edit {
    padding: var(--espacement-xs) var(--espacement-sm);
    border: 1px solid var(--couleur-bordure);
    border-radius: var(--rayon-bordure-sm);
  }
  
  /* === Contenu détaillé === */
  .detail-content {
    flex: 1;
    padding: var(--espacement-lg);
  }
  
  .content-display {
    line-height: 1.6;
    color: var(--couleur-texte-principal);
    white-space: pre-wrap;
  }
  
  .textarea-edit {
    width: 100%;
    height: 300px;
    padding: var(--espacement-md);
    border: 1px solid var(--couleur-bordure);
    border-radius: var(--rayon-bordure-md);
    resize: vertical;
    font-family: inherit;
    line-height: 1.6;
  }
  
  /* === Tags détaillés === */
  .detail-tags {
    padding: var(--espacement-lg);
    border-top: 1px solid var(--couleur-bordure);
  }
  
  .detail-tags h4 {
    margin: 0 0 var(--espacement-md) 0;
    font-size: 1rem;
    color: var(--couleur-texte-principal);
  }
  
  .tags-container {
    display: flex;
    flex-wrap: wrap;
    gap: var(--espacement-sm);
  }
  
  .tag-item {
    background: var(--couleur-fond-secondaire);
    padding: var(--espacement-xs) var(--espacement-sm);
    border-radius: var(--rayon-bordure-xl);
    font-size: 0.85rem;
    color: var(--couleur-principale);
    display: flex;
    align-items: center;
    gap: var(--espacement-xs);
  }
  
  .btn-remove-tag {
    background: none;
    border: none;
    color: var(--couleur-erreur);
    cursor: pointer;
    font-weight: bold;
  }
  
  .input-nouveau-tag {
    padding: var(--espacement-xs) var(--espacement-sm);
    border: 1px dashed var(--couleur-bordure);
    border-radius: var(--rayon-bordure-xl);
    font-size: 0.85rem;
    min-width: 100px;
  }
  
  /* === Responsive === */
  @media (max-width: 1200px) {
    .notes-container {
      grid-template-columns: 300px 1fr;
    }
    
    .detail-panel {
      position: fixed;
      right: 0;
      top: 0;
      height: 100vh;
      z-index: 1000;
      box-shadow: var(--ombre-xl);
    }
  }
  
  /* Bouton de fermeture */
  .btn-close {
    background: none;
    border: none;
    font-size: 1.2rem;
    cursor: pointer;
    opacity: 0.6;
    transition: all var(--transition-rapide);
    padding: var(--espacement-xs);
    border-radius: var(--rayon-bordure-sm);
    color: var(--couleur-texte-secondaire);
  }
  
  .btn-close:hover {
    opacity: 1;
    background: rgba(231, 76, 60, 0.1);
    color: var(--couleur-erreur);
  }
  
  @media (max-width: 768px) {
    .notes-container {
      grid-template-columns: 1fr;
    }
    
    .sidebar-notes {
      position: fixed;
      left: 0;
      top: 0;
      height: 100vh;
      z-index: 1001;
      box-shadow: var(--ombre-xl);
    }
    
    .notes-grid {
      grid-template-columns: 1fr;
    }
  }
</style>