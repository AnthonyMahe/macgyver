<!--
  Dashboard moderne MacGyver - Thème sombre élégant
  
  Interface moderne avec :
  - Salutation dynamique
  - Statistiques en temps réel
  - Accès rapide aux outils
  - Animations fluides
  - Composants réutilisables
-->

<script>
  import { onMount } from 'svelte';
  import { BoutonPrincipal, CarteContenu, IndicateurChargement } from '$lib/composants';
  
  let currentTime = new Date();
  let greeting = '';
  let chargementStats = true;
  
  // État des statistiques
  let statistiques = {
    pomodorosAujourdhui: 0,
    notesTotal: 0,
    tachesEnCours: 0,
    tachesTerminees: 0,
    imagesTraitees: 0,
    pdfsTraites: 0
  };
  
  onMount(() => {
    // Mise à jour de l'heure et salutation
    const updateTime = () => {
      currentTime = new Date();
      const hour = currentTime.getHours();
      
      if (hour < 12) {
        greeting = 'Bonjour';
      } else if (hour < 18) {
        greeting = 'Bon après-midi';
      } else {
        greeting = 'Bonsoir';
      }
    };
    
    updateTime();
    const interval = setInterval(updateTime, 1000);
    
    // Charger les statistiques
    chargerStatistiques();
    
    return () => clearInterval(interval);
  });
  
  async function chargerStatistiques() {
    chargementStats = true;
    
    // Simuler un délai de chargement pour montrer l'indicateur
    await new Promise(resolve => setTimeout(resolve, 1000));
    
    try {
      // Statistiques Pomodoro
      const pomodoroStats = JSON.parse(localStorage.getItem('macgyver_pomodoro_stats') || '{"pomodorosTermines": 0}');
      const pomodorosAujourdhui = pomodoroStats.pomodorosTermines || 0;
      
      // Statistiques Notes
      const notes = JSON.parse(localStorage.getItem('macgyver_notes') || '[]');
      const notesTotal = notes.length;
      
      // Statistiques TodoList
      const taches = JSON.parse(localStorage.getItem('productivapp_taches') || '[]');
      const tachesEnCours = taches.filter(t => !t.terminee).length;
      const tachesTerminees = taches.filter(t => t.terminee).length;
      
      // Statistiques Images (simulées pour l'instant)
      const imagesTraitees = parseInt(localStorage.getItem('macgyver_images_count') || '0');
      const pdfsTraites = parseInt(localStorage.getItem('macgyver_pdfs_count') || '0');
      
      statistiques = {
        pomodorosAujourdhui,
        notesTotal,
        tachesEnCours,
        tachesTerminees,
        imagesTraitees,
        pdfsTraites
      };
    } catch (error) {
      console.error('Erreur lors du chargement des statistiques:', error);
    } finally {
      chargementStats = false;
    }
  }
  
  const outils = [
    {
      id: 'pomodoro',
      icon: '🍅',
      title: 'Timer Pomodoro',
      description: 'Boostez votre concentration avec la technique Pomodoro. Sessions de travail optimisées.',
      link: '/pomodoro',
      gradient: 'from-red-500 to-orange-500'
    },
    {
      id: 'notes',
      icon: '📝',
      title: 'Prise de Notes',
      description: 'Capturez et organisez vos idées. Interface markdown avec recherche intégrée.',
      link: '/notes',
      gradient: 'from-blue-500 to-cyan-500'
    },
    {
      id: 'todos',
      icon: '✅',
      title: 'Gestionnaire de Tâches',
      description: 'Organisez votre workflow. Priorités, échéances et suivi de progression.',
      link: '/todos',
      gradient: 'from-green-500 to-emerald-500'
    },
    {
      id: 'images',
      icon: '🖼️',
      title: 'Studio d\'Images',
      description: 'Convertissez et manipulez vos images. Suppression de fond et optimisation.',
      link: '/images',
      gradient: 'from-purple-500 to-pink-500'
    },
    {
      id: 'pdf',
      icon: '📄',
      title: 'Manipulateur PDF',
      description: 'Fusionnez, divisez et analysez vos documents PDF en toute simplicité.',
      link: '/pdf',
      gradient: 'from-indigo-500 to-purple-500'
    }
  ];
</script>

<svelte:head>
  <title>MacGyver - Suite d'Outils Moderne</title>
</svelte:head>

<div class="dashboard">
  
  <!-- En-tête avec salutation -->
  <div class="dashboard-header">
    <div class="greeting-section">
      <h1 class="greeting-title">{greeting} 👋</h1>
      <p class="greeting-subtitle">Prêt à être productif ? Choisissez votre outil.</p>
      <div class="current-time">
        {currentTime.toLocaleTimeString('fr-FR', { 
          hour: '2-digit', 
          minute: '2-digit',
          second: '2-digit'
        })}
      </div>
    </div>
    
    <div class="hero-visual">
      <div class="floating-icons">
        <span class="floating-icon" style="--delay: 0s">🔧</span>
        <span class="floating-icon" style="--delay: 0.5s">⚡</span>
        <span class="floating-icon" style="--delay: 1s">🚀</span>
      </div>
    </div>
  </div>
  
  <!-- Statistiques rapides avec composants réutilisables -->
  <CarteContenu 
    titre="📊 Aperçu Rapide" 
    sousTitre="Vos activités récentes"
    variante="primary"
    chargement={chargementStats}
  >
    {#if chargementStats}
      <IndicateurChargement 
        message="Chargement des statistiques..." 
        type="dots" 
        couleur="primary"
        centrer={true}
      />
    {:else}
      <div class="stats-cards">
        <div class="stat-card">
          <div class="stat-icon">⏱️</div>
          <div class="stat-content">
            <div class="stat-number">{statistiques.pomodorosAujourdhui}</div>
            <div class="stat-label">Pomodoros aujourd'hui</div>
          </div>
          <div class="stat-glow stat-glow-red"></div>
        </div>
        
        <div class="stat-card">
          <div class="stat-icon">📋</div>
          <div class="stat-content">
            <div class="stat-number">{statistiques.tachesTerminees}</div>
            <div class="stat-label">Tâches complétées</div>
          </div>
          <div class="stat-glow stat-glow-green"></div>
        </div>
        
        <div class="stat-card">
          <div class="stat-icon">📝</div>
          <div class="stat-content">
            <div class="stat-number">{statistiques.notesTotal}</div>
            <div class="stat-label">Notes créées</div>
          </div>
          <div class="stat-glow stat-glow-blue"></div>
        </div>
        
        <div class="stat-card">
          <div class="stat-icon">🎨</div>
          <div class="stat-content">
            <div class="stat-number">{statistiques.imagesTraitees}</div>
            <div class="stat-label">Images traitées</div>
          </div>
          <div class="stat-glow stat-glow-purple"></div>
        </div>
      </div>
    {/if}
    
    <svelte:fragment slot="actions">
      <BoutonPrincipal 
        texte="Actualiser" 
        icone="🔄" 
        variante="ghost" 
        taille="sm"
        on:click={chargerStatistiques}
      />
    </svelte:fragment>
  </CarteContenu>
  
  <!-- Grille des outils -->
  <div class="tools-section">
    <div class="section-header">
      <h2>🛠️ Vos Outils</h2>
      <p>Choisissez l'outil parfait pour votre tâche</p>
    </div>
    
    <div class="tools-grid">
      {#each outils as outil}
        <a href={outil.link} class="tool-card" data-tool={outil.id}>
          <div class="tool-card-inner">
            <div class="tool-header">
              <div class="tool-icon">{outil.icon}</div>
            </div>
            
            <div class="tool-content">
              <h3 class="tool-title">{outil.title}</h3>
              <p class="tool-description">{outil.description}</p>
            </div>
            
            <div class="tool-footer">
              <span class="tool-cta">Ouvrir l'outil</span>
              <span class="tool-arrow">→</span>
            </div>
          </div>
          
          <div class="tool-glow"></div>
        </a>
      {/each}
    </div>
  </div>
  
</div>

<style>
  .dashboard {
    min-height: 100vh;
    padding: var(--espacement-2xl);
    display: flex;
    flex-direction: column;
    gap: var(--espacement-2xl);
    position: relative;
    z-index: 1;
  }
  
  /* === En-tête avec salutation === */
  .dashboard-header {
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
  
  .dashboard-header::before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: linear-gradient(135deg, 
      rgba(99, 102, 241, 0.05) 0%, 
      rgba(139, 92, 246, 0.05) 50%, 
      rgba(6, 182, 212, 0.05) 100%);
    pointer-events: none;
  }
  
  .greeting-section {
    position: relative;
    z-index: 1;
  }
  
  .greeting-title {
    font-size: 3rem;
    font-weight: 800;
    background: linear-gradient(135deg, var(--couleur-principale) 0%, var(--couleur-secondaire) 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
    margin-bottom: var(--espacement-md);
    line-height: 1.2;
  }
  
  .greeting-subtitle {
    font-size: 1.2rem;
    color: var(--couleur-texte-secondaire);
    margin-bottom: var(--espacement-lg);
    line-height: 1.5;
  }
  
  .current-time {
    font-family: var(--police-monospace);
    font-size: 1.5rem;
    font-weight: 600;
    color: var(--couleur-principale);
    background: var(--couleur-fond-tertiaire);
    padding: var(--espacement-sm) var(--espacement-lg);
    border-radius: var(--rayon-bordure-md);
    display: inline-block;
    border: 1px solid var(--couleur-bordure);
  }
  
  .hero-visual {
    position: relative;
    z-index: 1;
  }
  
  .floating-icons {
    display: flex;
    gap: var(--espacement-lg);
  }
  
  .floating-icon {
    font-size: 3rem;
    animation: float 3s ease-in-out infinite;
    animation-delay: var(--delay);
    filter: drop-shadow(0 4px 8px rgba(99, 102, 241, 0.3));
  }
  
  @keyframes float {
    0%, 100% { transform: translateY(0px); }
    50% { transform: translateY(-10px); }
  }
  
  /* === Statistiques rapides === */
  .quick-stats {
    background: var(--couleur-fond-secondaire);
    border-radius: var(--rayon-bordure-xl);
    padding: var(--espacement-2xl);
    border: 1px solid var(--couleur-bordure);
  }
  
  .stats-header {
    text-align: center;
    margin-bottom: var(--espacement-2xl);
  }
  
  .stats-header h2 {
    font-size: 2rem;
    font-weight: 700;
    color: var(--couleur-texte-principal);
    margin-bottom: var(--espacement-sm);
  }
  
  .stats-header p {
    color: var(--couleur-texte-secondaire);
    font-size: 1.1rem;
  }
  
  .stats-cards {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: var(--espacement-xl);
  }
  
  .stat-card {
    background: var(--couleur-fond-tertiaire);
    border: 1px solid var(--couleur-bordure);
    border-radius: var(--rayon-bordure-lg);
    padding: var(--espacement-xl);
    display: flex;
    align-items: center;
    gap: var(--espacement-lg);
    transition: all var(--transition-normale);
    position: relative;
    overflow: hidden;
  }
  
  .stat-card:hover {
    transform: translateY(-4px);
    border-color: var(--couleur-bordure-claire);
  }
  
  .stat-icon {
    font-size: 2.5rem;
    position: relative;
    z-index: 1;
  }
  
  .stat-content {
    position: relative;
    z-index: 1;
  }
  
  .stat-number {
    font-size: 2.5rem;
    font-weight: 800;
    color: var(--couleur-texte-principal);
    line-height: 1;
    margin-bottom: var(--espacement-xs);
  }
  
  .stat-label {
    font-size: 0.9rem;
    color: var(--couleur-texte-secondaire);
    font-weight: 500;
  }
  
  .stat-glow {
    position: absolute;
    top: 50%;
    right: -20px;
    width: 100px;
    height: 100px;
    border-radius: 50%;
    opacity: 0.1;
    transition: opacity var(--transition-normale);
  }
  
  .stat-card:hover .stat-glow {
    opacity: 0.2;
  }
  
  .stat-glow-red { background: radial-gradient(circle, #ef4444 0%, transparent 70%); }
  .stat-glow-green { background: radial-gradient(circle, #10b981 0%, transparent 70%); }
  .stat-glow-blue { background: radial-gradient(circle, #06b6d4 0%, transparent 70%); }
  .stat-glow-purple { background: radial-gradient(circle, #8b5cf6 0%, transparent 70%); }
  
  /* === Section des outils === */
  .tools-section {
    background: var(--couleur-fond-secondaire);
    border-radius: var(--rayon-bordure-xl);
    padding: var(--espacement-2xl);
    border: 1px solid var(--couleur-bordure);
  }
  
  .section-header {
    text-align: center;
    margin-bottom: var(--espacement-2xl);
  }
  
  .section-header h2 {
    font-size: 2rem;
    font-weight: 700;
    color: var(--couleur-texte-principal);
    margin-bottom: var(--espacement-sm);
  }
  
  .section-header p {
    color: var(--couleur-texte-secondaire);
    font-size: 1.1rem;
  }
  
  .tools-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    gap: var(--espacement-xl);
  }
  
  .tool-card {
    text-decoration: none;
    color: inherit;
    position: relative;
    display: block;
    transition: all var(--transition-normale);
  }
  
  .tool-card:hover {
    transform: translateY(-8px);
  }
  
  .tool-card-inner {
    background: var(--couleur-fond-tertiaire);
    border: 1px solid var(--couleur-bordure);
    border-radius: var(--rayon-bordure-lg);
    padding: var(--espacement-xl);
    height: 100%;
    display: flex;
    flex-direction: column;
    position: relative;
    z-index: 1;
    transition: all var(--transition-normale);
  }
  
  .tool-card:hover .tool-card-inner {
    border-color: var(--couleur-bordure-claire);
    background: var(--couleur-fond-quaternaire);
  }
  
  .tool-header {
    margin-bottom: var(--espacement-lg);
  }
  
  .tool-icon {
    font-size: 3rem;
    display: block;
    margin-bottom: var(--espacement-md);
    filter: drop-shadow(0 2px 4px rgba(0, 0, 0, 0.3));
  }
  
  .tool-content {
    flex: 1;
    margin-bottom: var(--espacement-lg);
  }
  
  .tool-title {
    font-size: 1.4rem;
    font-weight: 700;
    color: var(--couleur-texte-principal);
    margin-bottom: var(--espacement-md);
    line-height: 1.3;
  }
  
  .tool-description {
    color: var(--couleur-texte-secondaire);
    line-height: 1.6;
    font-size: 0.95rem;
  }
  
  .tool-footer {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding-top: var(--espacement-md);
    border-top: 1px solid var(--couleur-bordure);
  }
  
  .tool-cta {
    font-weight: 600;
    color: var(--couleur-principale);
    font-size: 0.9rem;
  }
  
  .tool-arrow {
    font-size: 1.2rem;
    color: var(--couleur-principale);
    transition: transform var(--transition-rapide);
  }
  
  .tool-card:hover .tool-arrow {
    transform: translateX(4px);
  }
  
  .tool-glow {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: linear-gradient(135deg, var(--couleur-principale) 0%, var(--couleur-secondaire) 100%);
    opacity: 0;
    border-radius: var(--rayon-bordure-lg);
    transition: opacity var(--transition-normale);
    z-index: 0;
  }
  
  .tool-card:hover .tool-glow {
    opacity: 0.05;
  }
  
  /* === Responsive === */
  @media (max-width: 768px) {
    .dashboard {
      padding: var(--espacement-lg);
      gap: var(--espacement-xl);
    }
    
    .dashboard-header {
      grid-template-columns: 1fr;
      text-align: center;
      padding: var(--espacement-xl);
    }
    
    .greeting-title {
      font-size: 2.5rem;
    }
    
    .stats-cards {
      grid-template-columns: repeat(2, 1fr);
    }
    
    .tools-grid {
      grid-template-columns: 1fr;
    }
  }
  
  @media (max-width: 480px) {
    .stats-cards {
      grid-template-columns: 1fr;
    }
    
    .greeting-title {
      font-size: 2rem;
    }
  }
</style>