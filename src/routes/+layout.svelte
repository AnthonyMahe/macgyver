<!--
  Layout principal de MacGyver
  
  Navigation entre les différents outils via une sidebar
-->

<script>
  import { page } from '$app/stores';
  import '../lib/styles/global.css';
  
  // Navigation items
  const navigationItems = [
    { 
      path: '/', 
      icon: '🏠', 
      label: 'Accueil',
      description: 'Vue d\'ensemble'
    },
    { 
      path: '/pomodoro', 
      icon: '🍅', 
      label: 'Pomodoro',
      description: 'Timer de productivité'
    },
    { 
      path: '/notes', 
      icon: '📝', 
      label: 'Notes',
      description: 'Prise de notes rapide'
    },
    { 
      path: '/todos', 
      icon: '✅', 
      label: 'TodoList',
      description: 'Gestion des tâches'
    },
    { 
      path: '/images', 
      icon: '🖼️', 
      label: 'Images',
      description: 'Convertisseur d\'images'
    },
    { 
      path: '/pdf', 
      icon: '📄', 
      label: 'PDF',
      description: 'Manipulateur de PDF'
    }
  ];
  
  // État de la sidebar
  let sidebarOuverte = true;
  
  function basculerSidebar() {
    sidebarOuverte = !sidebarOuverte;
  }
</script>

<svelte:head>
  <title>MacGyver - Outils de Productivité</title>
  <meta name="description" content="MacGyver - Suite d'outils de productivité avec Pomodoro, Notes et TodoList" />
</svelte:head>

<div class="app-container">
  
  <!-- Sidebar de navigation -->
  <nav class="sidebar" class:sidebar-fermee={!sidebarOuverte}>
    
    <!-- Logo et titre -->
    <div class="sidebar-header">
      <div class="logo">
        <span class="logo-icon">🔧</span>
        {#if sidebarOuverte}
          <span class="logo-text">MacGyver</span>
        {/if}
      </div>
      
      <button 
        class="btn-toggle-sidebar" 
        on:click={basculerSidebar}
        title={sidebarOuverte ? 'Réduire' : 'Agrandir'}
      >
        {sidebarOuverte ? '◀' : '▶'}
      </button>
    </div>
    
    <!-- Menu de navigation -->
    <ul class="nav-menu">
      {#each navigationItems as item}
        <li class="nav-item">
          <a 
            href={item.path}
            class="nav-link"
            class:active={$page.url.pathname === item.path}
            title={item.description}
          >
            <span class="nav-icon">{item.icon}</span>
            {#if sidebarOuverte}
              <span class="nav-label">{item.label}</span>
            {/if}
          </a>
        </li>
      {/each}
    </ul>
    
  </nav>
  
  <!-- Contenu principal -->
  <main class="main-content" class:main-etendu={!sidebarOuverte}>
    <slot />
  </main>
  
</div>

<style>
  .app-container {
    display: flex;
    height: 100vh;
    background: var(--couleur-fond-principal);
  }
  
  /* === Sidebar === */
  .sidebar {
    width: 280px;
    background: var(--couleur-fond-secondaire);
    border-right: 1px solid var(--couleur-bordure);
    display: flex;
    flex-direction: column;
    transition: all var(--transition-normale);
    box-shadow: var(--ombre-lg);
    position: relative;
  }
  
  .sidebar::before {
    content: '';
    position: absolute;
    top: 0;
    right: 0;
    width: 1px;
    height: 100%;
    background: linear-gradient(180deg, var(--couleur-principale) 0%, var(--couleur-secondaire) 100%);
    opacity: 0.3;
  }
  
  .sidebar-fermee {
    width: 80px;
  }
  
  .sidebar-header {
    padding: var(--espacement-xl);
    border-bottom: 1px solid var(--couleur-bordure);
    display: flex;
    align-items: center;
    justify-content: space-between;
    background: linear-gradient(135deg, var(--couleur-fond-tertiaire) 0%, var(--couleur-fond-secondaire) 100%);
    position: relative;
  }
  
  .sidebar-header::after {
    content: '';
    position: absolute;
    bottom: 0;
    left: 0;
    right: 0;
    height: 1px;
    background: linear-gradient(90deg, transparent 0%, var(--couleur-principale) 50%, transparent 100%);
    opacity: 0.5;
  }
  
  .logo {
    display: flex;
    align-items: center;
    gap: var(--espacement-md);
  }
  
  .logo-icon {
    font-size: 2rem;
    background: linear-gradient(135deg, var(--couleur-principale) 0%, var(--couleur-secondaire) 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
    filter: drop-shadow(0 2px 4px rgba(99, 102, 241, 0.3));
  }
  
  .logo-text {
    font-size: 1.5rem;
    font-weight: 700;
    color: var(--couleur-texte-principal);
    background: linear-gradient(135deg, var(--couleur-texte-principal) 0%, var(--couleur-texte-secondaire) 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
  }
  
  .btn-toggle-sidebar {
    background: var(--couleur-fond-tertiaire);
    border: 1px solid var(--couleur-bordure);
    font-size: 1rem;
    cursor: pointer;
    padding: var(--espacement-sm);
    border-radius: var(--rayon-bordure-md);
    color: var(--couleur-texte-secondaire);
    transition: all var(--transition-rapide);
    display: flex;
    align-items: center;
    justify-content: center;
    width: 36px;
    height: 36px;
  }
  
  .btn-toggle-sidebar:hover {
    background: var(--couleur-fond-quaternaire);
    color: var(--couleur-principale);
    border-color: var(--couleur-principale);
    transform: scale(1.05);
  }
  
  /* === Navigation === */
  .nav-menu {
    list-style: none;
    margin: 0;
    padding: var(--espacement-lg) var(--espacement-md);
    flex: 1;
  }
  
  .nav-item {
    margin-bottom: var(--espacement-sm);
  }
  
  .nav-link {
    display: flex;
    align-items: center;
    gap: var(--espacement-md);
    padding: var(--espacement-md) var(--espacement-lg);
    color: var(--couleur-texte-secondaire);
    text-decoration: none;
    transition: all var(--transition-rapide);
    border-radius: var(--rayon-bordure-md);
    position: relative;
    overflow: hidden;
  }
  
  .nav-link::before {
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
  
  .nav-link:hover {
    color: var(--couleur-texte-principal);
    transform: translateX(4px);
  }
  
  .nav-link:hover::before {
    opacity: 0.1;
  }
  
  .nav-link.active {
    background: linear-gradient(135deg, rgba(99, 102, 241, 0.15) 0%, rgba(139, 92, 246, 0.15) 100%);
    color: var(--couleur-principale-claire);
    border: 1px solid rgba(99, 102, 241, 0.3);
    box-shadow: var(--ombre-principale);
  }
  
  .nav-link.active::before {
    opacity: 0.2;
  }
  
  .nav-icon {
    font-size: 1.3rem;
    min-width: 28px;
    text-align: center;
    position: relative;
    z-index: 1;
  }
  
  .nav-label {
    font-weight: 500;
    position: relative;
    z-index: 1;
  }
  
  /* === Contenu principal === */
  .main-content {
    flex: 1;
    overflow-y: auto;
    transition: margin-left var(--transition-normale);
    background: var(--couleur-fond-principal);
    position: relative;
  }
  
  .main-content::before {
    content: '';
    position: fixed;
    top: 0;
    left: 280px;
    right: 0;
    height: 100vh;
    background: 
      radial-gradient(circle at 20% 20%, rgba(99, 102, 241, 0.05) 0%, transparent 50%),
      radial-gradient(circle at 80% 80%, rgba(139, 92, 246, 0.05) 0%, transparent 50%),
      radial-gradient(circle at 40% 60%, rgba(6, 182, 212, 0.03) 0%, transparent 50%);
    pointer-events: none;
    z-index: 0;
    transition: left var(--transition-normale);
  }
  
  .main-etendu::before {
    left: 80px;
  }
  
  .main-etendu {
    margin-left: 0;
  }
  
  /* === Responsive === */
  @media (max-width: 768px) {
    .sidebar {
      position: fixed;
      top: 0;
      left: 0;
      height: 100vh;
      z-index: 1000;
      transform: translateX(-100%);
    }
    
    .sidebar:not(.sidebar-fermee) {
      transform: translateX(0);
    }
    
    .main-content {
      margin-left: 0;
      width: 100%;
    }
  }
</style>