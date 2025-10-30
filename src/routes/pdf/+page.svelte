<!--
  Manipulateur de PDF MacGyver
  
  Interface pour manipuler des fichiers PDF :
  - Fusion de plusieurs PDFs
  - Division d'un PDF en pages
  - Extraction d'informations
  - Aperçu des métadonnées
-->

<script>
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/tauri';
  import { open, save } from '@tauri-apps/api/dialog';
  import { BoutonPrincipal } from '$lib/composants';
  
  // État de l'application
  let ongletActif = 'fusion'; // 'fusion', 'division', 'info'
  let enTraitement = false;
  let resultatsOperations = [];
  
  // === État pour la fusion ===
  let fichiersPourFusion = [];
  let optionsFusion = {
    inclure_signets: true,
    optimiser: true,
    ajouter_page_titre: false
  };
  
  // === État pour la division ===
  let fichierPourDivision = null;
  let infoPdfDivision = null;
  let optionsDivision = {
    mode: 'pages',
    plages: null,
    prefixe_nom: 'page'
  };
  
  // === État pour les informations ===
  let fichierPourInfo = null;
  let infoPdf = null;
  
  // === Fonctions pour la fusion ===
  
  async function ajouterFichierFusion() {
    try {
      const chemins = await open({
        multiple: true,
        filters: [{
          name: 'PDF',
          extensions: ['pdf']
        }]
      });
      
      if (chemins && chemins.length > 0) {
        for (const chemin of chemins) {
          // Vérifier si le fichier n'est pas déjà dans la liste
          if (!fichiersPourFusion.some(f => f.chemin === chemin)) {
            try {
              const info = await invoke('obtenir_info_pdf', { cheminFichier: chemin });
              fichiersPourFusion = [...fichiersPourFusion, {
                chemin: chemin,
                nom: chemin.split(/[\\/]/).pop(),
                pages: info.nombre_pages,
                taille: info.taille_octets
              }];
            } catch (error) {
              console.error('Erreur lors de l\'analyse du PDF:', error);
              alert('Impossible d\'analyser le PDF : ' + error);
            }
          }
        }
      }
    } catch (error) {
      console.error('Erreur lors de la sélection:', error);
    }
  }
  
  function supprimerFichierFusion(index) {
    fichiersPourFusion = fichiersPourFusion.filter((_, i) => i !== index);
  }
  
  function deplacerFichierFusion(index, direction) {
    if (direction === 'haut' && index > 0) {
      [fichiersPourFusion[index], fichiersPourFusion[index - 1]] = 
      [fichiersPourFusion[index - 1], fichiersPourFusion[index]];
    } else if (direction === 'bas' && index < fichiersPourFusion.length - 1) {
      [fichiersPourFusion[index], fichiersPourFusion[index + 1]] = 
      [fichiersPourFusion[index + 1], fichiersPourFusion[index]];
    }
    fichiersPourFusion = [...fichiersPourFusion]; // Déclencher la réactivité
  }
  
  async function fusionnerPdfs() {
    if (fichiersPourFusion.length < 2) {
      alert('Il faut au moins 2 fichiers PDF pour effectuer une fusion');
      return;
    }
    
    try {
      const cheminSortie = await save({
        defaultPath: 'document_fusionne.pdf',
        filters: [{
          name: 'PDF',
          extensions: ['pdf']
        }]
      });
      
      if (!cheminSortie) return;
      
      enTraitement = true;
      
      const chemins = fichiersPourFusion.map(f => f.chemin);
      const resultat = await invoke('fusionner_pdfs', {
        cheminsFichiers: chemins,
        cheminSortie: cheminSortie,
        options: optionsFusion
      });
      
      resultatsOperations = [resultat, ...resultatsOperations];
      alert('Fusion réussie ! ' + resultat.message);
      
    } catch (error) {
      console.error('Erreur de fusion:', error);
      alert('Erreur lors de la fusion : ' + error);
    } finally {
      enTraitement = false;
    }
  }
  
  // === Fonctions pour la division ===
  
  async function selectionnerFichierDivision() {
    try {
      const chemin = await open({
        multiple: false,
        filters: [{
          name: 'PDF',
          extensions: ['pdf']
        }]
      });
      
      if (chemin) {
        try {
          infoPdfDivision = await invoke('obtenir_info_pdf', { cheminFichier: chemin });
          fichierPourDivision = {
            chemin: chemin,
            nom: chemin.split(/[\\/]/).pop()
          };
        } catch (error) {
          console.error('Erreur lors de l\'analyse du PDF:', error);
          alert('Impossible d\'analyser le PDF : ' + error);
        }
      }
    } catch (error) {
      console.error('Erreur lors de la sélection:', error);
    }
  }
  
  async function diviserPdf() {
    if (!fichierPourDivision || !infoPdfDivision) {
      alert('Veuillez d\'abord sélectionner un fichier PDF');
      return;
    }
    
    try {
      const dossierSortie = await open({
        directory: true
      });
      
      if (!dossierSortie) return;
      
      enTraitement = true;
      
      const resultat = await invoke('diviser_pdf', {
        cheminFichier: fichierPourDivision.chemin,
        dossierSortie: dossierSortie,
        options: optionsDivision
      });
      
      resultatsOperations = [resultat, ...resultatsOperations];
      alert('Division réussie ! ' + resultat.message);
      
    } catch (error) {
      console.error('Erreur de division:', error);
      alert('Erreur lors de la division : ' + error);
    } finally {
      enTraitement = false;
    }
  }
  
  // === Fonctions pour les informations ===
  
  async function selectionnerFichierInfo() {
    try {
      const chemin = await open({
        multiple: false,
        filters: [{
          name: 'PDF',
          extensions: ['pdf']
        }]
      });
      
      if (chemin) {
        try {
          infoPdf = await invoke('obtenir_info_pdf', { cheminFichier: chemin });
          fichierPourInfo = {
            chemin: chemin,
            nom: chemin.split(/[\\/]/).pop()
          };
        } catch (error) {
          console.error('Erreur lors de l\'analyse du PDF:', error);
          alert('Impossible d\'analyser le PDF : ' + error);
        }
      }
    } catch (error) {
      console.error('Erreur lors de la sélection:', error);
    }
  }
  
  // === Utilitaires ===
  
  function formaterTaille(octets) {
    if (octets === 0) return '0 B';
    const k = 1024;
    const tailles = ['B', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(octets) / Math.log(k));
    return parseFloat((octets / Math.pow(k, i)).toFixed(2)) + ' ' + tailles[i];
  }
  
  function formaterDate(dateStr) {
    if (!dateStr) return 'Non disponible';
    try {
      // Les dates PDF sont souvent au format D:YYYYMMDDHHmmSSOHH'mm'
      if (dateStr.startsWith('D:')) {
        const cleanDate = dateStr.substring(2, 16); // YYYYMMDDHHMMSS
        const year = cleanDate.substring(0, 4);
        const month = cleanDate.substring(4, 6);
        const day = cleanDate.substring(6, 8);
        const hour = cleanDate.substring(8, 10);
        const minute = cleanDate.substring(10, 12);
        return `${day}/${month}/${year} ${hour}:${minute}`;
      }
      return dateStr;
    } catch {
      return dateStr;
    }
  }
</script>

<svelte:head>
  <title>Manipulateur PDF - MacGyver</title>
</svelte:head>

<div class="pdf-container">
  
  <!-- En-tête -->
  <div class="header">
    <h1>📄 Manipulateur PDF</h1>
    <p>Fusionnez, divisez et analysez vos documents PDF facilement</p>
  </div>
  
  <!-- Onglets -->
  <div class="tabs">
    <button 
      class="tab"
      class:active={ongletActif === 'fusion'}
      on:click={() => ongletActif = 'fusion'}
    >
      🔗 Fusion
    </button>
    <button 
      class="tab"
      class:active={ongletActif === 'division'}
      on:click={() => ongletActif = 'division'}
    >
      ✂️ Division
    </button>
    <button 
      class="tab"
      class:active={ongletActif === 'info'}
      on:click={() => ongletActif = 'info'}
    >
      ℹ️ Informations
    </button>
  </div>
  
  <!-- Contenu des onglets -->
  <div class="tab-content">
    
    <!-- Onglet Fusion -->
    {#if ongletActif === 'fusion'}
      <div class="fusion-section">
        <h3>🔗 Fusionner des PDFs</h3>
        <p>Sélectionnez plusieurs fichiers PDF à fusionner en un seul document.</p>
        
        <!-- Liste des fichiers -->
        <div class="file-list">
          {#each fichiersPourFusion as fichier, index}
            <div class="file-item">
              <div class="file-info">
                <div class="file-name">📄 {fichier.nom}</div>
                <div class="file-details">
                  {fichier.pages} pages • {formaterTaille(fichier.taille)}
                </div>
              </div>
              <div class="file-actions">
                <button 
                  class="btn-small"
                  on:click={() => deplacerFichierFusion(index, 'haut')}
                  disabled={index === 0}
                >
                  ↑
                </button>
                <button 
                  class="btn-small"
                  on:click={() => deplacerFichierFusion(index, 'bas')}
                  disabled={index === fichiersPourFusion.length - 1}
                >
                  ↓
                </button>
                <button 
                  class="btn-small btn-danger"
                  on:click={() => supprimerFichierFusion(index)}
                >
                  🗑️
                </button>
              </div>
            </div>
          {/each}
          
          {#if fichiersPourFusion.length === 0}
            <div class="empty-state">
              <p>Aucun fichier sélectionné</p>
            </div>
          {/if}
        </div>
        
        <!-- Actions -->
        <div class="actions">
          <BoutonPrincipal 
            texte="Ajouter des PDFs"
            icone="➕"
            variante="secondary"
            taille="md"
            on:click={ajouterFichierFusion}
          />
          
          {#if fichiersPourFusion.length >= 2}
            <button 
              class="btn-primary"
              on:click={fusionnerPdfs}
              disabled={enTraitement}
            >
              {#if enTraitement}
                🔄 Fusion en cours...
              {:else}
                🚀 Fusionner ({fichiersPourFusion.length} fichiers)
              {/if}
            </button>
          {/if}
        </div>
        
        <!-- Options de fusion -->
        <div class="options">
          <h4>Options de fusion</h4>
          <label>
            <input type="checkbox" bind:checked={optionsFusion.inclure_signets} />
            Inclure les signets (bookmarks)
          </label>
          <label>
            <input type="checkbox" bind:checked={optionsFusion.optimiser} />
            Optimiser le fichier de sortie
          </label>
          <label>
            <input type="checkbox" bind:checked={optionsFusion.ajouter_page_titre} />
            Ajouter une page de titre
          </label>
        </div>
      </div>
    {/if}
    
    <!-- Onglet Division -->
    {#if ongletActif === 'division'}
      <div class="division-section">
        <h3>✂️ Diviser un PDF</h3>
        <p>Divisez un PDF en pages individuelles ou en plages personnalisées.</p>
        
        <!-- Sélection du fichier -->
        <div class="file-selection">
          {#if !fichierPourDivision}
            <BoutonPrincipal 
              texte="Sélectionner un PDF à diviser"
              icone="📁"
              variante="primary"
              taille="lg"
              largeurComplete={true}
              on:click={selectionnerFichierDivision}
            />
          {:else}
            <div class="selected-file">
              <div class="file-info">
                <div class="file-name">📄 {fichierPourDivision.nom}</div>
                {#if infoPdfDivision}
                  <div class="file-details">
                    {infoPdfDivision.nombre_pages} pages • {formaterTaille(infoPdfDivision.taille_octets)}
                    • Version {infoPdfDivision.version}
                  </div>
                {/if}
              </div>
              <button 
                class="btn-change"
                on:click={() => { fichierPourDivision = null; infoPdfDivision = null; }}
              >
                Changer
              </button>
            </div>
          {/if}
        </div>
        
        {#if fichierPourDivision && infoPdfDivision}
          <!-- Options de division -->
          <div class="options">
            <h4>Options de division</h4>
            
            <div class="option-group">
              <label for="mode-division">Mode de division</label>
              <select id="mode-division" bind:value={optionsDivision.mode}>
                <option value="pages">Une page par fichier</option>
                <option value="plages" disabled>Plages personnalisées (bientôt disponible)</option>
              </select>
            </div>
            
            <div class="option-group">
              <label for="prefixe-nom">Préfixe des noms de fichiers</label>
              <input id="prefixe-nom" type="text" bind:value={optionsDivision.prefixe_nom} />
            </div>
          </div>
          
          <!-- Action de division -->
          <div class="actions">
            <button 
              class="btn-primary"
              on:click={diviserPdf}
              disabled={enTraitement}
            >
              {#if enTraitement}
                🔄 Division en cours...
              {:else}
                ✂️ Diviser en {infoPdfDivision.nombre_pages} fichiers
              {/if}
            </button>
          </div>
        {/if}
      </div>
    {/if}
    
    <!-- Onglet Informations -->
    {#if ongletActif === 'info'}
      <div class="info-section">
        <h3>ℹ️ Informations PDF</h3>
        <p>Analysez les métadonnées et propriétés d'un fichier PDF.</p>
        
        <!-- Sélection du fichier -->
        <div class="file-selection">
          {#if !fichierPourInfo}
            <BoutonPrincipal 
              texte="Sélectionner un PDF à analyser"
              icone="📁"
              variante="primary"
              taille="lg"
              largeurComplete={true}
              on:click={selectionnerFichierInfo}
            />
          {:else}
            <div class="selected-file">
              <div class="file-info">
                <div class="file-name">📄 {fichierPourInfo.nom}</div>
              </div>
              <button 
                class="btn-change"
                on:click={() => { fichierPourInfo = null; infoPdf = null; }}
              >
                Changer
              </button>
            </div>
          {/if}
        </div>
        
        {#if infoPdf}
          <!-- Informations détaillées -->
          <div class="pdf-info">
            <div class="info-grid">
              <div class="info-item">
                <div class="info-label">Nombre de pages</div>
                <div class="info-value">{infoPdf.nombre_pages}</div>
              </div>
              
              <div class="info-item">
                <div class="info-label">Taille du fichier</div>
                <div class="info-value">{formaterTaille(infoPdf.taille_octets)}</div>
              </div>
              
              <div class="info-item">
                <div class="info-label">Version PDF</div>
                <div class="info-value">{infoPdf.version}</div>
              </div>
              
              <div class="info-item">
                <div class="info-label">Analysé le</div>
                <div class="info-value">{new Date(infoPdf.horodatage).toLocaleString('fr-FR')}</div>
              </div>
            </div>
            
            <!-- Métadonnées -->
            <div class="metadata">
              <h4>📋 Métadonnées</h4>
              <div class="metadata-grid">
                <div class="metadata-item">
                  <div class="metadata-label">Titre</div>
                  <div class="metadata-value">{infoPdf.metadonnees.titre || 'Non spécifié'}</div>
                </div>
                
                <div class="metadata-item">
                  <div class="metadata-label">Auteur</div>
                  <div class="metadata-value">{infoPdf.metadonnees.auteur || 'Non spécifié'}</div>
                </div>
                
                <div class="metadata-item">
                  <div class="metadata-label">Sujet</div>
                  <div class="metadata-value">{infoPdf.metadonnees.sujet || 'Non spécifié'}</div>
                </div>
                
                <div class="metadata-item">
                  <div class="metadata-label">Créateur</div>
                  <div class="metadata-value">{infoPdf.metadonnees.createur || 'Non spécifié'}</div>
                </div>
                
                <div class="metadata-item">
                  <div class="metadata-label">Producteur</div>
                  <div class="metadata-value">{infoPdf.metadonnees.producteur || 'Non spécifié'}</div>
                </div>
                
                <div class="metadata-item">
                  <div class="metadata-label">Date de création</div>
                  <div class="metadata-value">{formaterDate(infoPdf.metadonnees.date_creation)}</div>
                </div>
                
                <div class="metadata-item">
                  <div class="metadata-label">Date de modification</div>
                  <div class="metadata-value">{formaterDate(infoPdf.metadonnees.date_modification)}</div>
                </div>
              </div>
            </div>
          </div>
        {/if}
      </div>
    {/if}
  </div>
  
  <!-- Historique des opérations -->
  {#if resultatsOperations.length > 0}
    <div class="history-section">
      <h3>📋 Historique des opérations</h3>
      
      <div class="history-list">
        {#each resultatsOperations as resultat}
          <div class="history-item">
            <div class="history-info">
              <div class="history-title">{resultat.message}</div>
              <div class="history-details">
                {resultat.pages_traitees} pages traitées • {resultat.temps_traitement_ms}ms
                • {new Date(resultat.horodatage).toLocaleString('fr-FR')}
              </div>
              {#if resultat.fichiers_sortie.length > 0}
                <div class="history-files">
                  Fichiers créés : {resultat.fichiers_sortie.length}
                </div>
              {/if}
            </div>
            <div class="history-status">
              {#if resultat.succes}
                <span class="status-success">✅ Réussi</span>
              {:else}
                <span class="status-error">❌ Échec</span>
              {/if}
            </div>
          </div>
        {/each}
      </div>
    </div>
  {/if}
  
</div>

<style>
  .pdf-container {
    max-width: 1000px;
    margin: 0 auto;
    padding: var(--espacement-xl);
    display: flex;
    flex-direction: column;
    gap: var(--espacement-xl);
  }
  
  /* En-tête */
  .header {
    text-align: center;
    margin-bottom: var(--espacement-lg);
  }
  
  .header h1 {
    color: var(--couleur-texte-principal);
    margin-bottom: var(--espacement-sm);
  }
  
  .header p {
    color: var(--couleur-texte-secondaire);
  }
  
  /* Onglets */
  .tabs {
    display: flex;
    background: var(--couleur-fond-secondaire);
    border: 1px solid var(--couleur-bordure);
    border-radius: var(--rayon-bordure-lg);
    box-shadow: var(--ombre-md);
    overflow: hidden;
  }
  
  .tab {
    flex: 1;
    padding: var(--espacement-lg);
    background: transparent;
    border: none;
    cursor: pointer;
    transition: all var(--transition-rapide);
    font-weight: 500;
    color: var(--couleur-texte-secondaire);
  }
  
  .tab:hover {
    background: var(--couleur-fond-tertiaire);
  }
  
  .tab.active {
    background: var(--couleur-principale);
    color: white;
  }
  
  /* Contenu des onglets */
  .tab-content {
    background: var(--couleur-fond-secondaire);
    border: 1px solid var(--couleur-bordure);
    padding: var(--espacement-xl);
    border-radius: var(--rayon-bordure-lg);
    box-shadow: var(--ombre-md);
  }
  
  .tab-content h3 {
    margin-bottom: var(--espacement-md);
    color: var(--couleur-texte-principal);
  }
  
  .tab-content p {
    color: var(--couleur-texte-secondaire);
    margin-bottom: var(--espacement-lg);
  }
  
  /* Liste de fichiers */
  .file-list {
    border: 2px dashed var(--couleur-bordure);
    border-radius: var(--rayon-bordure-md);
    padding: var(--espacement-lg);
    margin-bottom: var(--espacement-lg);
    min-height: 200px;
  }
  
  .file-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: var(--espacement-md);
    background: var(--couleur-fond-tertiaire);
    border: 1px solid var(--couleur-bordure);
    border-radius: var(--rayon-bordure-sm);
    margin-bottom: var(--espacement-sm);
    transition: all var(--transition-rapide);
  }
  
  .file-item:hover {
    background: var(--couleur-fond-quaternaire);
    border-color: var(--couleur-bordure-claire);
  }
  
  .file-info {
    flex: 1;
  }
  
  .file-name {
    font-weight: 500;
    color: var(--couleur-texte-principal);
    margin-bottom: var(--espacement-xs);
  }
  
  .file-details {
    font-size: 0.9rem;
    color: var(--couleur-texte-secondaire);
  }
  
  .file-actions {
    display: flex;
    gap: var(--espacement-xs);
  }
  
  .empty-state {
    text-align: center;
    color: var(--couleur-texte-secondaire);
    padding: var(--espacement-2xl);
  }
  
  /* Sélection de fichier */
  .file-selection {
    margin-bottom: var(--espacement-lg);
  }
  
  .btn-select-large {
    width: 100%;
    padding: var(--espacement-2xl);
    background: var(--couleur-fond-secondaire);
    border: 2px dashed var(--couleur-bordure);
    border-radius: var(--rayon-bordure-md);
    cursor: pointer;
    transition: all var(--transition-rapide);
    font-size: 1.1rem;
    color: var(--couleur-texte-principal);
  }
  
  .btn-select-large:hover {
    border-color: var(--couleur-principale);
    background: rgba(52, 152, 219, 0.05);
  }
  
  .selected-file {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: var(--espacement-lg);
    background: var(--couleur-fond-secondaire);
    border-radius: var(--rayon-bordure-md);
  }
  
  /* Boutons */
  .btn-primary {
    padding: var(--espacement-md) var(--espacement-xl);
    background: var(--couleur-principale);
    color: white;
    border: none;
    border-radius: var(--rayon-bordure-md);
    cursor: pointer;
    font-weight: 600;
    transition: all var(--transition-rapide);
  }
  
  .btn-primary:hover:not(:disabled) {
    background: var(--couleur-principale-foncee);
    transform: translateY(-1px);
  }
  
  .btn-primary:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }
  
  .btn-secondary {
    padding: var(--espacement-md) var(--espacement-xl);
    background: var(--couleur-texte-secondaire);
    color: white;
    border: none;
    border-radius: var(--rayon-bordure-md);
    cursor: pointer;
    transition: all var(--transition-rapide);
  }
  
  .btn-secondary:hover {
    background: var(--couleur-texte-principal);
  }
  
  .btn-small {
    padding: var(--espacement-xs) var(--espacement-sm);
    background: var(--couleur-texte-secondaire);
    color: white;
    border: none;
    border-radius: var(--rayon-bordure-sm);
    cursor: pointer;
    font-size: 0.8rem;
  }
  
  .btn-small:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
  
  .btn-danger {
    background: var(--couleur-erreur) !important;
  }
  
  .btn-change {
    padding: var(--espacement-sm) var(--espacement-md);
    background: var(--couleur-texte-secondaire);
    color: white;
    border: none;
    border-radius: var(--rayon-bordure-sm);
    cursor: pointer;
  }
  
  /* Actions */
  .actions {
    display: flex;
    gap: var(--espacement-md);
    margin-bottom: var(--espacement-lg);
  }
  
  /* Options */
  .options {
    background: var(--couleur-fond-secondaire);
    padding: var(--espacement-lg);
    border-radius: var(--rayon-bordure-md);
    margin-bottom: var(--espacement-lg);
  }
  
  .options h4 {
    margin-bottom: var(--espacement-md);
    color: var(--couleur-texte-principal);
  }
  
  .options label {
    display: flex;
    align-items: center;
    gap: var(--espacement-sm);
    margin-bottom: var(--espacement-sm);
    cursor: pointer;
  }
  
  .option-group {
    margin-bottom: var(--espacement-md);
  }
  
  .option-group label {
    display: block;
    margin-bottom: var(--espacement-xs);
    font-weight: 500;
  }
  
  .option-group select,
  .option-group input[type="text"] {
    width: 100%;
    padding: var(--espacement-sm);
    background: var(--couleur-fond-tertiaire);
    border: 1px solid var(--couleur-bordure);
    border-radius: var(--rayon-bordure-sm);
    color: var(--couleur-texte-principal);
    transition: all var(--transition-rapide);
  }
  
  .option-group select:focus,
  .option-group input[type="text"]:focus {
    outline: none;
    border-color: var(--couleur-principale);
    box-shadow: 0 0 0 3px rgba(99, 102, 241, 0.15);
    background: var(--couleur-fond-secondaire);
    transform: translateY(-1px);
  }
  
  /* Informations PDF */
  .pdf-info {
    margin-top: var(--espacement-lg);
  }
  
  .info-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: var(--espacement-md);
    margin-bottom: var(--espacement-xl);
  }
  
  .info-item {
    background: var(--couleur-fond-tertiaire);
    border: 1px solid var(--couleur-bordure);
    padding: var(--espacement-md);
    border-radius: var(--rayon-bordure-sm);
    transition: all var(--transition-rapide);
  }
  
  .info-item:hover {
    background: var(--couleur-fond-quaternaire);
    border-color: var(--couleur-bordure-claire);
  }
  
  .info-label {
    font-size: 0.9rem;
    color: var(--couleur-texte-secondaire);
    margin-bottom: var(--espacement-xs);
  }
  
  .info-value {
    font-weight: 600;
    color: var(--couleur-texte-principal);
  }
  
  /* Métadonnées */
  .metadata {
    background: var(--couleur-fond-tertiaire);
    border: 1px solid var(--couleur-bordure);
    padding: var(--espacement-lg);
    border-radius: var(--rayon-bordure-md);
  }
  
  .metadata h4 {
    margin-bottom: var(--espacement-md);
    color: var(--couleur-texte-principal);
  }
  
  .metadata-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    gap: var(--espacement-md);
  }
  
  .metadata-item {
    display: flex;
    flex-direction: column;
    gap: var(--espacement-xs);
  }
  
  .metadata-label {
    font-size: 0.9rem;
    font-weight: 500;
    color: var(--couleur-texte-secondaire);
  }
  
  .metadata-value {
    color: var(--couleur-texte-principal);
    word-break: break-word;
  }
  
  /* Historique */
  .history-section {
    background: var(--couleur-fond-secondaire);
    border: 1px solid var(--couleur-bordure);
    padding: var(--espacement-xl);
    border-radius: var(--rayon-bordure-lg);
    box-shadow: var(--ombre-md);
  }
  
  .history-section h3 {
    margin-bottom: var(--espacement-lg);
    color: var(--couleur-texte-principal);
  }
  
  .history-list {
    display: flex;
    flex-direction: column;
    gap: var(--espacement-md);
  }
  
  .history-item {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    padding: var(--espacement-md);
    background: var(--couleur-fond-tertiaire);
    border: 1px solid var(--couleur-bordure);
    border-radius: var(--rayon-bordure-md);
    transition: all var(--transition-rapide);
  }
  
  .history-item:hover {
    background: var(--couleur-fond-quaternaire);
    border-color: var(--couleur-bordure-claire);
  }
  
  .history-title {
    font-weight: 500;
    color: var(--couleur-texte-principal);
    margin-bottom: var(--espacement-xs);
  }
  
  .history-details {
    font-size: 0.9rem;
    color: var(--couleur-texte-secondaire);
  }
  
  .history-files {
    font-size: 0.9rem;
    color: var(--couleur-principale);
    margin-top: var(--espacement-xs);
  }
  
  .status-success {
    color: var(--couleur-succes);
    font-weight: 500;
  }
  
  .status-error {
    color: var(--couleur-erreur);
    font-weight: 500;
  }
  
  /* Responsive */
  @media (max-width: 768px) {
    .pdf-container {
      padding: var(--espacement-lg);
    }
    
    .tabs {
      flex-direction: column;
    }
    
    .actions {
      flex-direction: column;
    }
    
    .file-item {
      flex-direction: column;
      align-items: flex-start;
      gap: var(--espacement-sm);
    }
    
    .selected-file {
      flex-direction: column;
      align-items: flex-start;
      gap: var(--espacement-sm);
    }
    
    .info-grid {
      grid-template-columns: 1fr;
    }
    
    .metadata-grid {
      grid-template-columns: 1fr;
    }
  }
</style>