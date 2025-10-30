<!--
  Page Pomodoro Timer - Thème sombre moderne
  
  Technique de productivité avec cycles de travail de 25 minutes
  suivis de pauses de 5 minutes, avec une pause longue de 15 minutes
  toutes les 4 sessions.
-->

<script>
  import { onMount, onDestroy } from 'svelte';
  import { BoutonPrincipal, CarteContenu, IndicateurChargement } from '$lib/composants';
  
  // États du timer
  let tempsRestant = 25 * 60; // 25 minutes en secondes
  let timerActif = false;
  let intervalId = null;
  
  // Types de sessions
  let typeSession = 'travail'; // 'travail', 'pause', 'pause-longue'
  let numeroSession = 1;
  let sessionsCompletees = 0;
  
  // Configuration
  const durees = {
    travail: 25 * 60,      // 25 minutes
    pause: 5 * 60,         // 5 minutes
    'pause-longue': 15 * 60 // 15 minutes
  };
  
  // Statistiques de la journée
  let statistiques = {
    pomodorosTermines: 0,
    tempsTotal: 0,
    pausesPrises: 0
  };
  
  // Sons et notifications
  let sonActive = true;
  
  // Formatage du temps
  function formaterTemps(secondes) {
    const minutes = Math.floor(secondes / 60);
    const secondesRestantes = secondes % 60;
    return `${minutes.toString().padStart(2, '0')}:${secondesRestantes.toString().padStart(2, '0')}`;
  }
  
  // Calcul du pourcentage de progression
  function calculerProgression() {
    const dureeTotal = durees[typeSession];
    return ((dureeTotal - tempsRestant) / dureeTotal) * 100;
  }
  
  // Démarrer/Arrêter le timer
  function toggleTimer() {
    if (timerActif) {
      pauserTimer();
    } else {
      demarrerTimer();
    }
  }
  
  function demarrerTimer() {
    timerActif = true;
    intervalId = setInterval(() => {
      tempsRestant--;
      
      if (tempsRestant <= 0) {
        terminerSession();
      }
    }, 1000);
  }
  
  function pauserTimer() {
    timerActif = false;
    if (intervalId) {
      clearInterval(intervalId);
      intervalId = null;
    }
  }
  
  function arreterTimer() {
    pauserTimer();
    reinitialiserSession();
  }
  
  function terminerSession() {
    pauserTimer();
    
    // Jouer un son de notification
    if (sonActive) {
      jouerSonNotification();
    }
    
    // Mettre à jour les statistiques
    if (typeSession === 'travail') {
      statistiques.pomodorosTermines++;
      sessionsCompletees++;
      statistiques.tempsTotal += durees.travail;
    } else {
      statistiques.pausesPrises++;
    }
    
    // Passer à la session suivante
    passerSessionSuivante();
  }
  
  function passerSessionSuivante() {
    if (typeSession === 'travail') {
      // Après le travail, pause courte ou longue
      if (sessionsCompletees % 4 === 0) {
        typeSession = 'pause-longue';
      } else {
        typeSession = 'pause';
      }
    } else {
      // Après une pause, retour au travail
      typeSession = 'travail';
      numeroSession++;
    }
    
    tempsRestant = durees[typeSession];
  }
  
  function reinitialiserSession() {
    typeSession = 'travail';
    tempsRestant = durees[typeSession];
    numeroSession = 1;
  }
  
  function jouerSonNotification() {
    // Simulation d'un son (dans une vraie app, on utiliserait l'API Audio)
    console.log('🔔 Session terminée !');
  }
  
  // Raccourcis clavier
  function gererRaccourcis(event) {
    if (event.code === 'Space') {
      event.preventDefault();
      toggleTimer();
    } else if (event.code === 'Escape') {
      event.preventDefault();
      arreterTimer();
    }
  }
  
  onMount(() => {
    // Charger les statistiques sauvegardées
    const statsString = localStorage.getItem('macgyver_pomodoro_stats');
    if (statsString) {
      statistiques = JSON.parse(statsString);
    }
    
    // Écouter les raccourcis clavier
    window.addEventListener('keydown', gererRaccourcis);
  });
  
  onDestroy(() => {
    // Nettoyer l'intervalle
    if (intervalId) {
      clearInterval(intervalId);
    }
    
    // Sauvegarder les statistiques
    localStorage.setItem('macgyver_pomodoro_stats', JSON.stringify(statistiques));
    
    // Supprimer l'écouteur d'événements
    window.removeEventListener('keydown', gererRaccourcis);
  });
  
  // Messages motivationnels
  const messagesMotivation = {
    travail: [
      "🎯 Concentrez-vous sur votre tâche !",
      "💪 Vous pouvez le faire !",
      "🚀 Restez dans le flow !",
      "⚡ L'énergie est là !"
    ],
    pause: [
      "😌 Prenez une pause bien méritée",
      "🌱 Rechargez vos batteries",
      "☕ Temps pour un café ?",
      "🧘 Détendez-vous un moment"
    ],
    'pause-longue': [
      "🎉 Excellente session ! Pause longue",
      "🌟 Vous avez bien travaillé !",
      "🏆 Temps pour une vraie pause",
      "💆 Reposez-vous bien"
    ]
  };
  
  function obtenirMessageMotivation() {
    const messages = messagesMotivation[typeSession];
    return messages[Math.floor(Math.random() * messages.length)];
  }
</script>

<svelte:head>
  <title>Pomodoro Timer - MacGyver</title>
</svelte:head>

<div class="pomodoro-container">
  
  <!-- En-tête -->
  <div class="header">
    <h1>🍅 Timer Pomodoro</h1>
    <p>Boostez votre concentration avec la technique Pomodoro</p>
  </div>
  
  <!-- Timer principal avec composants modernes -->
  <CarteContenu 
    titre="Timer Principal" 
    icone="⏱️"
    variante="primary"
    elevation="lg"
  >
    <div class="timer-section">
    
    <!-- Indicateur de session -->
    <div class="session-info">
      <div class="session-type" class:type-travail={typeSession === 'travail'} 
           class:type-pause={typeSession === 'pause'} 
           class:type-pause-longue={typeSession === 'pause-longue'}>
        {#if typeSession === 'travail'}
          🍅 Session de travail #{numeroSession}
        {:else if typeSession === 'pause'}
          ☕ Pause courte
        {:else}
          🌟 Pause longue
        {/if}
      </div>
      
      <div class="motivation-message">
        {obtenirMessageMotivation()}
      </div>
    </div>
    
    <!-- Timer circulaire -->
    <div class="timer-circle">
      <svg class="progress-ring" width="300" height="300">
        <circle
          class="progress-ring-background"
          stroke="#e0e0e0"
          stroke-width="8"
          fill="transparent"
          r="140"
          cx="150"
          cy="150"
        />
        <circle
          class="progress-ring-progress"
          stroke={typeSession === 'travail' ? '#e74c3c' : typeSession === 'pause' ? '#3498db' : '#9b59b6'}
          stroke-width="8"
          fill="transparent"
          r="140"
          cx="150"
          cy="150"
          stroke-dasharray="879.646"
          stroke-dashoffset={879.646 - (879.646 * calculerProgression()) / 100}
          transform="rotate(-90 150 150)"
        />
      </svg>
      
      <div class="timer-display">
        <div class="time-remaining">{formaterTemps(tempsRestant)}</div>
        <div class="time-label">
          {#if typeSession === 'travail'}
            Travail
          {:else if typeSession === 'pause'}
            Pause
          {:else}
            Pause longue
          {/if}
        </div>
      </div>
    </div>
    
    <!-- Contrôles du timer -->
    <div class="timer-controls">
      <BoutonPrincipal 
        texte={timerActif ? 'Pause' : 'Démarrer'}
        icone={timerActif ? '⏸️' : '▶️'}
        variante="primary"
        taille="lg"
        on:click={toggleTimer}
      />
      
      <BoutonPrincipal 
        texte="Arrêter"
        icone="⏹️"
        variante="secondary"
        taille="md"
        desactive={!timerActif && tempsRestant === durees[typeSession]}
        on:click={arreterTimer}
      />
      
      <BoutonPrincipal 
        texte="Suivant"
        icone="⏭️"
        variante="ghost"
        taille="md"
        desactive={timerActif}
        on:click={passerSessionSuivante}
      />
    </div>
    
    <!-- Raccourcis -->
    <div class="shortcuts-info">
      <span>Raccourcis : <kbd>Espace</kbd> = Play/Pause • <kbd>Échap</kbd> = Arrêter</span>
    </div>
    
  </div>
  </CarteContenu>
  
  <!-- Panneau latéral -->
  <div class="sidebar-panel">
    
    <!-- Statistiques du jour -->
    <div class="stats-panel">
      <h3>📊 Aujourd'hui</h3>
      
      <div class="stat-item">
        <span class="stat-icon">🍅</span>
        <div class="stat-content">
          <div class="stat-number">{statistiques.pomodorosTermines}</div>
          <div class="stat-label">Pomodoros terminés</div>
        </div>
      </div>
      
      <div class="stat-item">
        <span class="stat-icon">⏱️</span>
        <div class="stat-content">
          <div class="stat-number">{Math.floor(statistiques.tempsTotal / 60)}</div>
          <div class="stat-label">Minutes de travail</div>
        </div>
      </div>
      
      <div class="stat-item">
        <span class="stat-icon">☕</span>
        <div class="stat-content">
          <div class="stat-number">{statistiques.pausesPrises}</div>
          <div class="stat-label">Pauses prises</div>
        </div>
      </div>
    </div>
    
    <!-- Configuration -->
    <div class="config-panel">
      <h3>⚙️ Configuration</h3>
      
      <div class="config-item">
        <label class="config-label">
          <input 
            type="checkbox" 
            bind:checked={sonActive}
          />
          Sons de notification
        </label>
      </div>
      
      <div class="config-item">
        <label for="duree-travail" class="config-label">
          Durée travail : {durees.travail / 60} min
        </label>
        <input 
          id="duree-travail"
          type="range" 
          min="15" 
          max="45" 
          bind:value={durees.travail}
          on:input={(e) => durees.travail = e.target.value * 60}
          disabled={timerActif}
        />
      </div>
      
      <div class="config-item">
        <label for="duree-pause" class="config-label">
          Durée pause : {durees.pause / 60} min
        </label>
        <input 
          id="duree-pause"
          type="range" 
          min="3" 
          max="10" 
          bind:value={durees.pause}
          on:input={(e) => durees.pause = e.target.value * 60}
          disabled={timerActif}
        />
      </div>
    </div>
    
    <!-- Conseils -->
    <div class="tips-panel">
      <h3>💡 Conseils Pomodoro</h3>
      <ul class="tips-list">
        <li>Choisissez une tâche spécifique avant de commencer</li>
        <li>Éliminez toutes les distractions</li>
        <li>Ne vérifiez pas vos messages pendant le travail</li>
        <li>Utilisez les pauses pour vous étirer</li>
        <li>Notez vos accomplissements</li>
      </ul>
    </div>
    
  </div>
  
</div>

<style>
  /* === Layout principal === */
  .pomodoro-container {
    display: grid;
    grid-template-columns: 1fr 350px;
    gap: var(--espacement-2xl);
    min-height: 100vh;
    max-width: 1200px;
    margin: 0 auto;
    padding: var(--espacement-xl);
  }
  
  /* === En-tête === */
  .header {
    grid-column: 1 / -1;
    text-align: center;
    margin-bottom: var(--espacement-xl);
  }
  
  .header h1 {
    font-size: 2.5rem;
    font-weight: 800;
    background: linear-gradient(135deg, #ef4444 0%, #f97316 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
    margin-bottom: var(--espacement-sm);
  }
  
  .header p {
    color: var(--couleur-texte-secondaire);
    font-size: 1.1rem;
  }
  
  /* === Section timer === */
  .timer-section {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: var(--espacement-xl);
    padding: var(--espacement-2xl);
  }
  
  /* === Info session === */
  .session-info {
    text-align: center;
    margin-bottom: var(--espacement-xl);
  }
  
  .session-type {
    font-size: 1.5rem;
    font-weight: 600;
    padding: var(--espacement-md) var(--espacement-xl);
    border-radius: var(--rayon-bordure-xl);
    margin-bottom: var(--espacement-md);
    border: 1px solid var(--couleur-bordure);
    background: var(--couleur-fond-tertiaire);
  }
  
  .type-travail {
    background: linear-gradient(135deg, rgba(239, 68, 68, 0.1) 0%, rgba(249, 115, 22, 0.1) 100%);
    color: #ef4444;
    border-color: rgba(239, 68, 68, 0.3);
  }
  
  .type-pause {
    background: linear-gradient(135deg, rgba(6, 182, 212, 0.1) 0%, rgba(59, 130, 246, 0.1) 100%);
    color: #06b6d4;
    border-color: rgba(6, 182, 212, 0.3);
  }
  
  .type-pause-longue {
    background: rgba(155, 89, 182, 0.1);
    color: #9b59b6;
  }
  
  .motivation-message {
    font-size: 1.1rem;
    color: var(--couleur-texte-secondaire);
    font-style: italic;
  }
  
  /* === Timer circulaire === */
  .timer-circle {
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
  }
  
  .progress-ring {
    transform: rotate(-90deg);
  }
  
  .progress-ring-progress {
    transition: stroke-dashoffset 0.3s ease;
  }
  
  .timer-display {
    position: absolute;
    text-align: center;
  }
  
  .time-remaining {
    font-size: 3.5rem;
    font-weight: 700;
    font-family: var(--police-monospace);
    color: var(--couleur-texte-principal);
    line-height: 1;
  }
  
  .time-label {
    font-size: 1.2rem;
    color: var(--couleur-texte-secondaire);
    margin-top: var(--espacement-sm);
  }
  
  /* === Contrôles === */
  .timer-controls {
    display: flex;
    gap: var(--espacement-lg);
    align-items: center;
  }
  
  .btn-control {
    padding: var(--espacement-md) var(--espacement-xl);
    border: none;
    border-radius: var(--rayon-bordure-md);
    font-size: 1.1rem;
    font-weight: 500;
    cursor: pointer;
    transition: all var(--transition-rapide);
    min-width: 120px;
  }
  
  .btn-primary {
    background: var(--couleur-principale);
    color: white;
  }
  
  .btn-primary:hover:not(:disabled) {
    background: var(--couleur-principale-foncee);
    transform: translateY(-2px);
  }
  
  .btn-pause {
    background: #e74c3c;
  }
  
  .btn-pause:hover:not(:disabled) {
    background: #c0392b;
  }
  
  .btn-secondary {
    background: var(--couleur-fond-secondaire);
    color: var(--couleur-texte-principal);
    border: 1px solid var(--couleur-bordure);
  }
  
  .btn-secondary:hover:not(:disabled) {
    background: var(--couleur-bordure);
  }
  
  .btn-control:disabled {
    opacity: 0.5;
    cursor: not-allowed;
    transform: none;
  }
  
  /* === Raccourcis === */
  .shortcuts-info {
    font-size: 0.9rem;
    color: var(--couleur-texte-secondaire);
    text-align: center;
  }
  
  kbd {
    background: var(--couleur-fond-secondaire);
    border: 1px solid var(--couleur-bordure);
    border-radius: 3px;
    padding: 2px 6px;
    font-family: var(--police-monospace);
    font-size: 0.8rem;
  }
  
  /* === Panneau latéral === */
  .sidebar-panel {
    display: flex;
    flex-direction: column;
    gap: var(--espacement-xl);
  }
  
  .stats-panel,
  .config-panel,
  .tips-panel {
    background: var(--couleur-fond-secondaire);
    border: 1px solid var(--couleur-bordure);
    padding: var(--espacement-xl);
    border-radius: var(--rayon-bordure-lg);
    box-shadow: var(--ombre-md);
  }
  
  .stats-panel h3,
  .config-panel h3,
  .tips-panel h3 {
    margin: 0 0 var(--espacement-lg) 0;
    color: var(--couleur-texte-principal);
    font-size: 1.1rem;
  }
  
  /* === Statistiques === */
  .stat-item {
    display: flex;
    align-items: center;
    gap: var(--espacement-md);
    margin-bottom: var(--espacement-md);
  }
  
  .stat-icon {
    font-size: 1.5rem;
  }
  
  .stat-number {
    font-size: 1.5rem;
    font-weight: 700;
    color: var(--couleur-principale);
  }
  
  .stat-label {
    font-size: 0.9rem;
    color: var(--couleur-texte-secondaire);
  }
  
  /* === Configuration === */
  .config-item {
    margin-bottom: var(--espacement-lg);
  }
  
  .config-label {
    display: block;
    margin-bottom: var(--espacement-sm);
    font-size: 0.9rem;
    color: var(--couleur-texte-principal);
    cursor: pointer;
  }
  
  input[type="range"] {
    width: 100%;
    margin-top: var(--espacement-sm);
  }
  
  input[type="checkbox"] {
    margin-right: var(--espacement-sm);
  }
  
  /* === Conseils === */
  .tips-list {
    list-style: none;
    padding: 0;
    margin: 0;
  }
  
  .tips-list li {
    padding: var(--espacement-sm) 0;
    font-size: 0.9rem;
    color: var(--couleur-texte-secondaire);
    border-bottom: 1px solid var(--couleur-bordure);
    position: relative;
    padding-left: var(--espacement-lg);
  }
  
  .tips-list li:last-child {
    border-bottom: none;
  }
  
  .tips-list li::before {
    content: '💡';
    position: absolute;
    left: 0;
  }
  
  /* === Responsive === */
  @media (max-width: 1024px) {
    .pomodoro-container {
      grid-template-columns: 1fr;
      gap: var(--espacement-xl);
    }
    
    .sidebar-panel {
      grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
      display: grid;
    }
  }
  
  @media (max-width: 768px) {
    .timer-section {
      padding: var(--espacement-lg);
    }
    
    .time-remaining {
      font-size: 2.5rem;
    }
    
    .timer-controls {
      flex-direction: column;
      gap: var(--espacement-md);
    }
    
    .btn-control {
      width: 100%;
      max-width: 200px;
    }
  }
</style>