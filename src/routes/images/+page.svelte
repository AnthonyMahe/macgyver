<!--
  Studio d'Images MacGyver - Interface moderne thème sombre
  
  Fonctionnalités :
  - Conversion d'images entre formats
  - Suppression de fond avec PNG transparent
  - Interface à onglets élégante
  - Barres de progression stylées
-->

<script>
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/tauri';
  import { open, save } from '@tauri-apps/api/dialog';
  import { BoutonPrincipal } from '$lib/composants';
  
  // État de l'application
  let ongletActif = 'conversion'; // 'conversion', 'suppression-fond'
  let fichierSelectionne = null;
  let infoImage = null;
  let formatsSupportes = [];
  let enTraitement = false;
  let resultatsConversion = [];
  
  // Options de conversion
  let optionsConversion = {
    format_sortie: 'PNG',
    qualite: 85,
    largeur_max: null,
    hauteur_max: null,
    conserver_ratio: true
  };
  
  // État pour la suppression de fond
  let fichierSuppressionFond = null;
  let infoImageSuppressionFond = null;
  let optionsSuppressionFond = {
    couleur_fond: '#FFFFFF',
    tolerance: 30,
    adoucir_bords: true,
    rayon_adoucissement: 2
  };
  
  onMount(async () => {
    // Charger les formats supportés
    try {
      formatsSupportes = await invoke('obtenir_formats_supportes');
    } catch (error) {
      console.error('Erreur lors du chargement des formats:', error);
    }
  });
  
  // === Fonctions pour la conversion ===
  
  async function ouvrirSelecteurFichier() {
    try {
      const cheminFichier = await open({
        multiple: false,
        filters: [{
          name: 'Images',
          extensions: ['jpg', 'jpeg', 'png', 'webp', 'bmp', 'tiff', 'gif']
        }]
      });
      
      if (cheminFichier) {
        await selectionnerFichierTauri(cheminFichier);
      }
    } catch (error) {
      console.error('Erreur lors de la sélection:', error);
    }
  }
  
  async function selectionnerFichierTauri(cheminFichier) {
    try {
      // Obtenir les informations de l'image
      infoImage = await invoke('obtenir_info_image', {
        cheminFichier: cheminFichier
      });
      
      // Créer un objet fichier simulé pour l'affichage
      fichierSelectionne = {
        name: cheminFichier.split(/[\\/]/).pop(),
        path: cheminFichier
      };
      
      // Définir des dimensions par défaut pour le redimensionnement
      optionsConversion.largeur_max = infoImage.largeur;
      optionsConversion.hauteur_max = infoImage.hauteur;
      
    } catch (error) {
      console.error('Erreur lors de l\'analyse de l\'image:', error);
      alert('Impossible d\'analyser cette image : ' + error);
      fichierSelectionne = null;
      infoImage = null;
    }
  }
  
  async function convertirImage() {
    if (!fichierSelectionne || !infoImage) return;
    
    try {
      // Demander où sauvegarder le fichier converti
      const nomBase = fichierSelectionne.name.replace(/\.[^/.]+$/, "");
      const extension = optionsConversion.format_sortie.toLowerCase() === 'jpeg' ? 'jpg' : optionsConversion.format_sortie.toLowerCase();
      
      const cheminSortie = await save({
        defaultPath: `${nomBase}_converti.${extension}`,
        filters: [{
          name: optionsConversion.format_sortie,
          extensions: [extension]
        }]
      });
      
      if (!cheminSortie) return;
      
      enTraitement = true;
      
      const resultat = await invoke('convertir_image', {
        cheminEntree: fichierSelectionne.path,
        cheminSortie: cheminSortie,
        options: optionsConversion
      });
      
      // Ajouter le résultat à la liste
      resultatsConversion = [resultat, ...resultatsConversion];
      
      // Incrémenter le compteur d'images traitées
      const count = parseInt(localStorage.getItem('macgyver_images_count') || '0') + 1;
      localStorage.setItem('macgyver_images_count', count.toString());
      
      alert('Conversion réussie ! ' + resultat.message);
      
    } catch (error) {
      console.error('Erreur de conversion:', error);
      alert('Erreur lors de la conversion : ' + error);
    } finally {
      enTraitement = false;
    }
  } 
 
  // === Fonctions pour la suppression de fond ===
  
  async function selectionnerFichierSuppressionFond() {
    try {
      const cheminFichier = await open({
        multiple: false,
        filters: [{
          name: 'Images',
          extensions: ['jpg', 'jpeg', 'png', 'webp', 'bmp', 'tiff', 'gif']
        }]
      });
      
      if (cheminFichier) {
        try {
          infoImageSuppressionFond = await invoke('obtenir_info_image', {
            cheminFichier: cheminFichier
          });
          
          fichierSuppressionFond = {
            name: cheminFichier.split(/[\\/]/).pop(),
            path: cheminFichier
          };
        } catch (error) {
          console.error('Erreur lors de l\'analyse de l\'image:', error);
          alert('Impossible d\'analyser cette image : ' + error);
        }
      }
    } catch (error) {
      console.error('Erreur lors de la sélection:', error);
    }
  }
  
  async function supprimerFondImage() {
    if (!fichierSuppressionFond || !infoImageSuppressionFond) {
      alert('Veuillez d\'abord sélectionner une image');
      return;
    }
    
    try {
      const nomBase = fichierSuppressionFond.name.replace(/\.[^/.]+$/, "");
      const cheminSortie = await save({
        defaultPath: `${nomBase}_sans_fond.png`,
        filters: [{
          name: 'PNG',
          extensions: ['png']
        }]
      });
      
      if (!cheminSortie) return;
      
      enTraitement = true;
      
      const resultat = await invoke('supprimer_fond_image', {
        cheminEntree: fichierSuppressionFond.path,
        cheminSortie: cheminSortie,
        options: optionsSuppressionFond
      });
      
      resultatsConversion = [resultat, ...resultatsConversion];
      
      // Incrémenter le compteur d'images traitées
      const count = parseInt(localStorage.getItem('macgyver_images_count') || '0') + 1;
      localStorage.setItem('macgyver_images_count', count.toString());
      
      alert('Suppression de fond réussie ! ' + resultat.message);
      
    } catch (error) {
      console.error('Erreur de suppression de fond:', error);
      alert('Erreur lors de la suppression de fond : ' + error);
    } finally {
      enTraitement = false;
    }
  }
  
  // Formatage de la taille de fichier
  function formaterTaille(octets) {
    if (octets === 0) return '0 B';
    const k = 1024;
    const tailles = ['B', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(octets) / Math.log(k));
    return parseFloat((octets / Math.pow(k, i)).toFixed(2)) + ' ' + tailles[i];
  }
</script>

<svelte:head>
  <title>Studio d'Images - MacGyver</title>
</svelte:head>

<div class="studio-container">
  
  <!-- En-tête moderne -->
  <div class="studio-header">
    <div class="header-content">
      <h1 class="studio-title">🎨 Studio d'Images</h1>
      <p class="studio-subtitle">Convertissez et manipulez vos images avec style</p>
    </div>
    <div class="header-visual">
      <div class="floating-shapes">
        <div class="shape shape-1"></div>
        <div class="shape shape-2"></div>
        <div class="shape shape-3"></div>
      </div>
    </div>
  </div>
  
  <!-- Onglets modernes -->
  <div class="modern-tabs">
    <button 
      class="tab-button"
      class:active={ongletActif === 'conversion'}
      on:click={() => ongletActif = 'conversion'}
    >
      <span class="tab-icon">🔄</span>
      <span class="tab-label">Conversion</span>
    </button>
    <button 
      class="tab-button"
      class:active={ongletActif === 'suppression-fond'}
      on:click={() => ongletActif = 'suppression-fond'}
    >
      <span class="tab-icon">🎨</span>
      <span class="tab-label">Suppression de fond</span>
    </button>
  </div>
  
  <!-- Contenu des onglets -->
  <div class="tab-content-wrapper">
    
    <!-- Onglet Conversion -->
    {#if ongletActif === 'conversion'}
      <div class="tool-panel">
        <div class="panel-header">
          <h2>🔄 Conversion d'Images</h2>
          <p>Convertissez vos images entre différents formats</p>
        </div>
        
        <!-- Zone de sélection -->
        <div class="file-drop-zone">
          {#if !fichierSelectionne}
            <div class="drop-content">
              <div class="drop-icon">📁</div>
              <h3>Sélectionnez votre image</h3>
              <p>Formats supportés : JPG, PNG, WebP, BMP, TIFF, GIF</p>
              <BoutonPrincipal 
                texte="Choisir un fichier"
                icone="📂"
                variante="primary"
                taille="md"
                on:click={ouvrirSelecteurFichier}
              />
            </div>
          {:else}
            <div class="file-preview">
              <div class="file-info">
                <div class="file-icon">🖼️</div>
                <div class="file-details">
                  <h3>{fichierSelectionne.name}</h3>
                  {#if infoImage}
                    <div class="file-meta">
                      <span class="meta-item">{infoImage.largeur} × {infoImage.hauteur}px</span>
                      <span class="meta-item">{formaterTaille(infoImage.taille_octets)}</span>
                      <span class="meta-item">{infoImage.format}</span>
                    </div>
                  {/if}
                </div>
              </div>
              <button 
                class="btn btn-ghost btn-sm"
                on:click={() => { fichierSelectionne = null; infoImage = null; }}
              >
                Changer
              </button>
            </div>
          {/if}
        </div>        

        <!-- Options de conversion -->
        {#if fichierSelectionne && infoImage}
          <div class="options-panel">
            <h3>⚙️ Options de conversion</h3>
            
            <div class="options-grid">
              <!-- Format de sortie -->
              <div class="form-group">
                <label for="format-sortie" class="form-label">Format de sortie</label>
                <select id="format-sortie" class="form-select" bind:value={optionsConversion.format_sortie}>
                  {#each formatsSupportes as format}
                    <option value={format}>{format}</option>
                  {/each}
                </select>
              </div>
              
              <!-- Qualité (pour JPEG) -->
              {#if optionsConversion.format_sortie === 'JPEG'}
                <div class="form-group">
                  <label for="qualite-jpeg" class="form-label">Qualité ({optionsConversion.qualite}%)</label>
                  <input 
                    id="qualite-jpeg"
                    type="range" 
                    min="10" 
                    max="100" 
                    bind:value={optionsConversion.qualite}
                    class="form-range"
                  />
                </div>
              {/if}
              
              <!-- Redimensionnement -->
              <div class="form-group">
                <label class="form-label checkbox-label">
                  <input 
                    type="checkbox" 
                    bind:checked={optionsConversion.conserver_ratio}
                    class="form-checkbox"
                  />
                  Conserver les proportions
                </label>
              </div>
              
              <div class="form-group">
                <label for="largeur-max" class="form-label">Largeur max (px)</label>
                <input 
                  id="largeur-max"
                  type="number" 
                  bind:value={optionsConversion.largeur_max}
                  min="1"
                  max="10000"
                  class="form-input"
                />
              </div>
              
              <div class="form-group">
                <label for="hauteur-max" class="form-label">Hauteur max (px)</label>
                <input 
                  id="hauteur-max"
                  type="number" 
                  bind:value={optionsConversion.hauteur_max}
                  min="1"
                  max="10000"
                  class="form-input"
                />
              </div>
            </div>
            
            <!-- Bouton de conversion -->
            <div class="action-section">
              <BoutonPrincipal 
                texte={enTraitement ? "Conversion en cours..." : "Convertir l'image"}
                icone={enTraitement ? "" : "🚀"}
                variante="primary"
                taille="lg"
                chargement={enTraitement}
                desactive={enTraitement}
                on:click={convertirImage}
              />
            </div>
          </div>
        {/if}
      </div>
    {/if}    

    <!-- Onglet Suppression de fond -->
    {#if ongletActif === 'suppression-fond'}
      <div class="tool-panel">
        <div class="panel-header">
          <h2>🎨 Suppression de fond</h2>
          <p>Supprimez le fond de vos images et exportez en PNG transparent</p>
        </div>
        
        <!-- Zone de sélection -->
        <div class="file-drop-zone">
          {#if !fichierSuppressionFond}
            <div class="drop-content">
              <div class="drop-icon">🎨</div>
              <h3>Sélectionnez votre image</h3>
              <p>L'image sera exportée en PNG avec fond transparent</p>
              <BoutonPrincipal 
                texte="Choisir un fichier"
                icone="📂"
                variante="primary"
                taille="md"
                on:click={selectionnerFichierSuppressionFond}
              />
            </div>
          {:else}
            <div class="file-preview">
              <div class="file-info">
                <div class="file-icon">🖼️</div>
                <div class="file-details">
                  <h3>{fichierSuppressionFond.name}</h3>
                  {#if infoImageSuppressionFond}
                    <div class="file-meta">
                      <span class="meta-item">{infoImageSuppressionFond.largeur} × {infoImageSuppressionFond.hauteur}px</span>
                      <span class="meta-item">{formaterTaille(infoImageSuppressionFond.taille_octets)}</span>
                      <span class="meta-item">{infoImageSuppressionFond.format}</span>
                    </div>
                  {/if}
                </div>
              </div>
              <button 
                class="btn btn-ghost btn-sm"
                on:click={() => { fichierSuppressionFond = null; infoImageSuppressionFond = null; }}
              >
                Changer
              </button>
            </div>
          {/if}
        </div>
        
        <!-- Options de suppression de fond -->
        {#if fichierSuppressionFond && infoImageSuppressionFond}
          <div class="options-panel">
            <h3>⚙️ Options de suppression</h3>
            
            <div class="options-grid">
              <!-- Couleur de fond -->
              <div class="form-group">
                <label for="couleur-fond" class="form-label">Couleur de fond à supprimer</label>
                <div class="color-input-group">
                  <input 
                    id="couleur-fond"
                    type="color" 
                    bind:value={optionsSuppressionFond.couleur_fond}
                    class="color-picker"
                  />
                  <input 
                    type="text" 
                    bind:value={optionsSuppressionFond.couleur_fond}
                    placeholder="#FFFFFF"
                    class="form-input"
                  />
                </div>
              </div>
              
              <!-- Tolérance -->
              <div class="form-group">
                <label for="tolerance" class="form-label">Tolérance ({optionsSuppressionFond.tolerance})</label>
                <input 
                  id="tolerance"
                  type="range" 
                  min="0" 
                  max="100" 
                  bind:value={optionsSuppressionFond.tolerance}
                  class="form-range"
                />
                <small class="form-help">Plus la valeur est élevée, plus de couleurs similaires seront supprimées</small>
              </div>
              
              <!-- Adoucissement -->
              <div class="form-group">
                <label class="form-label checkbox-label">
                  <input 
                    type="checkbox" 
                    bind:checked={optionsSuppressionFond.adoucir_bords}
                    class="form-checkbox"
                  />
                  Adoucir les bords
                </label>
                
                {#if optionsSuppressionFond.adoucir_bords}
                  <div class="sub-option">
                    <label for="rayon-adoucissement" class="form-label">Rayon d'adoucissement ({optionsSuppressionFond.rayon_adoucissement})</label>
                    <input 
                      id="rayon-adoucissement"
                      type="range" 
                      min="1" 
                      max="10" 
                      bind:value={optionsSuppressionFond.rayon_adoucissement}
                      class="form-range"
                    />
                  </div>
                {/if}
              </div>
            </div>
            
            <!-- Bouton de suppression -->
            <div class="action-section">
              <BoutonPrincipal 
                texte={enTraitement ? "Suppression en cours..." : "Supprimer le fond"}
                icone={enTraitement ? "" : "🎨"}
                variante="primary"
                taille="lg"
                chargement={enTraitement}
                desactive={enTraitement}
                on:click={supprimerFondImage}
              />
            </div>
          </div>
        {/if}
      </div>
    {/if}
    
  </div>  
  
  <!-- Historique des opérations -->
  {#if resultatsConversion.length > 0}
    <div class="history-panel">
      <div class="panel-header">
        <h2>📋 Historique des opérations</h2>
        <p>{resultatsConversion.length} opération{resultatsConversion.length > 1 ? 's' : ''} effectuée{resultatsConversion.length > 1 ? 's' : ''}</p>
      </div>
      
      <div class="history-list">
        {#each resultatsConversion as resultat, index}
          <div class="history-item" style="--delay: {index * 0.1}s">
            <div class="history-icon">
              {#if resultat.format_sortie === 'PNG' && resultat.message.includes('fond')}
                🎨
              {:else}
                🔄
              {/if}
            </div>
            <div class="history-content">
              <div class="history-title">
                {resultat.fichier_origine.split(/[\\/]/).pop()} → {resultat.format_sortie}
              </div>
              <div class="history-details">
                <span class="detail-item">{formaterTaille(resultat.taille_avant)} → {formaterTaille(resultat.taille_apres)}</span>
                {#if resultat.reduction_pourcent > 0}
                  <span class="detail-item reduction">-{resultat.reduction_pourcent.toFixed(1)}%</span>
                {:else if resultat.reduction_pourcent < 0}
                  <span class="detail-item augmentation">+{Math.abs(resultat.reduction_pourcent).toFixed(1)}%</span>
                {/if}
              </div>
            </div>
            <div class="history-status">
              {#if resultat.succes}
                <span class="badge badge-success">✅ Réussi</span>
              {:else}
                <span class="badge badge-error">❌ Échec</span>
              {/if}
            </div>
          </div>
        {/each}
      </div>
    </div>
  {/if}
  
</div>

<style>
  .studio-container {
    min-height: 100vh;
    padding: var(--espacement-2xl);
    display: flex;
    flex-direction: column;
    gap: var(--espacement-2xl);
    position: relative;
    z-index: 1;
  }
  
  /* === En-tête moderne === */
  .studio-header {
    display: grid;
    grid-template-columns: 1fr auto;
    gap: var(--espacement-2xl);
    align-items: center;
    padding: var(--espacement-2xl);
    background: var(--couleur-fond-secondaire);
    border-radius: var(--rayon-bordure-xl);
    border: 1px solid var(--couleur-bordure);
    position: relative;
    overflow: hidden;
  }
  
  .studio-header::before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: linear-gradient(135deg, 
      rgba(139, 92, 246, 0.05) 0%, 
      rgba(236, 72, 153, 0.05) 100%);
    pointer-events: none;
  }
  
  .header-content {
    position: relative;
    z-index: 1;
  }
  
  .studio-title {
    font-size: 2.5rem;
    font-weight: 800;
    background: linear-gradient(135deg, var(--couleur-secondaire) 0%, var(--couleur-accent-rose) 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
    margin-bottom: var(--espacement-sm);
    line-height: 1.2;
  }
  
  .studio-subtitle {
    font-size: 1.1rem;
    color: var(--couleur-texte-secondaire);
    line-height: 1.5;
  }
  
  .header-visual {
    position: relative;
    z-index: 1;
  }
  
  .floating-shapes {
    display: flex;
    gap: var(--espacement-md);
  }
  
  .shape {
    width: 60px;
    height: 60px;
    border-radius: 50%;
    animation: float 4s ease-in-out infinite;
    filter: blur(1px);
  }
  
  .shape-1 {
    background: linear-gradient(135deg, var(--couleur-secondaire), var(--couleur-accent-rose));
    animation-delay: 0s;
  }
  
  .shape-2 {
    background: linear-gradient(135deg, var(--couleur-accent-rose), var(--couleur-accent-amber));
    animation-delay: 1s;
  }
  
  .shape-3 {
    background: linear-gradient(135deg, var(--couleur-accent-amber), var(--couleur-secondaire));
    animation-delay: 2s;
  }
  
  /* === Onglets modernes === */
  .modern-tabs {
    display: flex;
    background: var(--couleur-fond-secondaire);
    border-radius: var(--rayon-bordure-lg);
    padding: var(--espacement-sm);
    border: 1px solid var(--couleur-bordure);
    gap: var(--espacement-sm);
  }
  
  .tab-button {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: var(--espacement-sm);
    padding: var(--espacement-md) var(--espacement-lg);
    background: transparent;
    border: none;
    border-radius: var(--rayon-bordure-md);
    cursor: pointer;
    transition: all var(--transition-rapide);
    color: var(--couleur-texte-secondaire);
    font-weight: 500;
    position: relative;
    overflow: hidden;
  }
  
  .tab-button::before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: linear-gradient(135deg, var(--couleur-principale) 0%, var(--couleur-secondaire) 100%);
    opacity: 0;
    transition: opacity var(--transition-rapide);
  }
  
  .tab-button:hover {
    color: var(--couleur-texte-principal);
  }
  
  .tab-button:hover::before {
    opacity: 0.1;
  }
  
  .tab-button.active {
    background: linear-gradient(135deg, var(--couleur-principale) 0%, var(--couleur-secondaire) 100%);
    color: white;
    box-shadow: var(--ombre-principale);
  }
  
  .tab-button.active::before {
    opacity: 0;
  }
  
  .tab-icon {
    font-size: 1.2rem;
    position: relative;
    z-index: 1;
  }
  
  .tab-label {
    position: relative;
    z-index: 1;
  }  
  
/* === Contenu des onglets === */
  .tab-content-wrapper {
    background: var(--couleur-fond-secondaire);
    border-radius: var(--rayon-bordure-xl);
    border: 1px solid var(--couleur-bordure);
    overflow: hidden;
  }
  
  .tool-panel {
    padding: var(--espacement-2xl);
  }
  
  .panel-header {
    text-align: center;
    margin-bottom: var(--espacement-2xl);
  }
  
  .panel-header h2 {
    font-size: 1.8rem;
    font-weight: 700;
    color: var(--couleur-texte-principal);
    margin-bottom: var(--espacement-sm);
  }
  
  .panel-header p {
    color: var(--couleur-texte-secondaire);
    font-size: 1rem;
  }
  
  /* === Zone de dépôt de fichier === */
  .file-drop-zone {
    background: var(--couleur-fond-tertiaire);
    border: 2px dashed var(--couleur-bordure);
    border-radius: var(--rayon-bordure-lg);
    padding: var(--espacement-2xl);
    margin-bottom: var(--espacement-2xl);
    transition: all var(--transition-rapide);
    position: relative;
    overflow: hidden;
  }
  
  .file-drop-zone::before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: linear-gradient(135deg, 
      rgba(99, 102, 241, 0.02) 0%, 
      rgba(139, 92, 246, 0.02) 100%);
    pointer-events: none;
  }
  
  .drop-content {
    text-align: center;
    position: relative;
    z-index: 1;
  }
  
  .drop-icon {
    font-size: 4rem;
    margin-bottom: var(--espacement-lg);
    opacity: 0.7;
  }
  
  .drop-content h3 {
    font-size: 1.4rem;
    font-weight: 600;
    color: var(--couleur-texte-principal);
    margin-bottom: var(--espacement-md);
  }
  
  .drop-content p {
    color: var(--couleur-texte-secondaire);
    margin-bottom: var(--espacement-xl);
  }
  
  /* === Aperçu du fichier === */
  .file-preview {
    display: flex;
    justify-content: space-between;
    align-items: center;
    background: var(--couleur-fond-quaternaire);
    border-radius: var(--rayon-bordure-md);
    padding: var(--espacement-lg);
    position: relative;
    z-index: 1;
  }
  
  .file-info {
    display: flex;
    align-items: center;
    gap: var(--espacement-lg);
    flex: 1;
  }
  
  .file-icon {
    font-size: 2.5rem;
    filter: drop-shadow(0 2px 4px rgba(0, 0, 0, 0.3));
  }
  
  .file-details h3 {
    font-size: 1.2rem;
    font-weight: 600;
    color: var(--couleur-texte-principal);
    margin-bottom: var(--espacement-xs);
  }
  
  .file-meta {
    display: flex;
    gap: var(--espacement-md);
    flex-wrap: wrap;
  }
  
  .meta-item {
    font-size: 0.85rem;
    color: var(--couleur-texte-tertiaire);
    background: var(--couleur-fond-tertiaire);
    padding: var(--espacement-xs) var(--espacement-sm);
    border-radius: var(--rayon-bordure-sm);
    border: 1px solid var(--couleur-bordure);
  }  

  /* === Panneau d'options === */
  .options-panel {
    background: var(--couleur-fond-tertiaire);
    border-radius: var(--rayon-bordure-lg);
    padding: var(--espacement-xl);
    border: 1px solid var(--couleur-bordure);
  }
  
  .options-panel h3 {
    font-size: 1.3rem;
    font-weight: 600;
    color: var(--couleur-texte-principal);
    margin-bottom: var(--espacement-lg);
    display: flex;
    align-items: center;
    gap: var(--espacement-sm);
  }
  
  .options-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    gap: var(--espacement-lg);
    margin-bottom: var(--espacement-xl);
  }
  
  /* === Éléments de formulaire === */
  .form-range {
    width: 100%;
    height: 6px;
    background: var(--couleur-fond-quaternaire);
    border-radius: var(--rayon-bordure-sm);
    outline: none;
    -webkit-appearance: none;
    margin: var(--espacement-sm) 0;
  }
  
  .form-range::-webkit-slider-thumb {
    -webkit-appearance: none;
    width: 20px;
    height: 20px;
    background: linear-gradient(135deg, var(--couleur-principale) 0%, var(--couleur-secondaire) 100%);
    border-radius: 50%;
    cursor: pointer;
    box-shadow: var(--ombre-sm);
  }
  
  .form-range::-moz-range-thumb {
    width: 20px;
    height: 20px;
    background: linear-gradient(135deg, var(--couleur-principale) 0%, var(--couleur-secondaire) 100%);
    border-radius: 50%;
    cursor: pointer;
    border: none;
    box-shadow: var(--ombre-sm);
  }
  
  .checkbox-label {
    display: flex !important;
    align-items: center;
    gap: var(--espacement-sm);
    cursor: pointer;
  }
  
  .form-checkbox {
    width: 18px;
    height: 18px;
    accent-color: var(--couleur-principale);
  }
  
  .color-input-group {
    display: flex;
    gap: var(--espacement-sm);
    align-items: center;
  }
  
  .color-picker {
    width: 50px;
    height: 40px;
    border: 1px solid var(--couleur-bordure);
    border-radius: var(--rayon-bordure-sm);
    cursor: pointer;
    background: var(--couleur-fond-quaternaire);
  }
  
  .form-help {
    font-size: 0.8rem;
    color: var(--couleur-texte-tertiaire);
    margin-top: var(--espacement-xs);
    display: block;
  }
  
  .sub-option {
    margin-top: var(--espacement-md);
    padding-left: var(--espacement-lg);
    border-left: 2px solid var(--couleur-bordure);
  }
  
  /* === Section d'action === */
  .action-section {
    text-align: center;
  }
  
  .btn-lg {
    padding: var(--espacement-lg) var(--espacement-2xl);
    font-size: 1.1rem;
    font-weight: 600;
  }
  
  .bg-removal {
    background: linear-gradient(135deg, var(--couleur-secondaire) 0%, var(--couleur-accent-rose) 100%) !important;
  }
  
  .bg-removal:hover:not(:disabled) {
    background: linear-gradient(135deg, var(--couleur-secondaire-claire) 0%, var(--couleur-accent-rose) 100%) !important;
  }
  
  .spinner {
    display: inline-block;
    width: 16px;
    height: 16px;
    border: 2px solid rgba(255, 255, 255, 0.3);
    border-top: 2px solid white;
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }
  
  @keyframes spin {
    0% { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
  }  
  
/* === Historique === */
  .history-panel {
    background: var(--couleur-fond-secondaire);
    border-radius: var(--rayon-bordure-xl);
    border: 1px solid var(--couleur-bordure);
    overflow: hidden;
  }
  
  .history-list {
    padding: var(--espacement-lg);
    display: flex;
    flex-direction: column;
    gap: var(--espacement-md);
  }
  
  .history-item {
    display: flex;
    align-items: center;
    gap: var(--espacement-lg);
    padding: var(--espacement-lg);
    background: var(--couleur-fond-tertiaire);
    border-radius: var(--rayon-bordure-md);
    border: 1px solid var(--couleur-bordure);
    transition: all var(--transition-rapide);
    animation: slideIn 0.3s ease-out;
    animation-delay: var(--delay);
    animation-fill-mode: both;
  }
  
  .history-item:hover {
    transform: translateX(4px);
    border-color: var(--couleur-bordure-claire);
    background: var(--couleur-fond-quaternaire);
  }
  
  @keyframes slideIn {
    from {
      opacity: 0;
      transform: translateY(20px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }
  
  .history-icon {
    font-size: 1.5rem;
    width: 40px;
    height: 40px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--couleur-fond-quaternaire);
    border-radius: var(--rayon-bordure-md);
    border: 1px solid var(--couleur-bordure);
  }
  
  .history-content {
    flex: 1;
  }
  
  .history-title {
    font-weight: 600;
    color: var(--couleur-texte-principal);
    margin-bottom: var(--espacement-xs);
    font-size: 0.95rem;
  }
  
  .history-details {
    display: flex;
    gap: var(--espacement-md);
    flex-wrap: wrap;
  }
  
  .detail-item {
    font-size: 0.8rem;
    color: var(--couleur-texte-tertiaire);
    background: var(--couleur-fond-quaternaire);
    padding: var(--espacement-xs) var(--espacement-sm);
    border-radius: var(--rayon-bordure-sm);
    border: 1px solid var(--couleur-bordure);
  }
  
  .reduction {
    color: var(--couleur-succes-claire) !important;
    background: rgba(16, 185, 129, 0.1) !important;
    border-color: rgba(16, 185, 129, 0.2) !important;
  }
  
  .augmentation {
    color: var(--couleur-avertissement-claire) !important;
    background: rgba(245, 158, 11, 0.1) !important;
    border-color: rgba(245, 158, 11, 0.2) !important;
  }
  
  /* === Responsive === */
  @media (max-width: 768px) {
    .studio-container {
      padding: var(--espacement-lg);
      gap: var(--espacement-xl);
    }
    
    .studio-header {
      grid-template-columns: 1fr;
      text-align: center;
      padding: var(--espacement-xl);
    }
    
    .studio-title {
      font-size: 2rem;
    }
    
    .modern-tabs {
      flex-direction: column;
    }
    
    .options-grid {
      grid-template-columns: 1fr;
    }
    
    .file-preview {
      flex-direction: column;
      align-items: flex-start;
      gap: var(--espacement-md);
    }
    
    .history-item {
      flex-direction: column;
      align-items: flex-start;
      gap: var(--espacement-sm);
    }
    
    .color-input-group {
      flex-direction: column;
      align-items: stretch;
    }
  }
</style>