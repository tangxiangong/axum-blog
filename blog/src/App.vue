<script setup lang="ts">
import { RouterView, useRoute } from 'vue-router'
import { computed } from 'vue'

const route = useRoute()
const isLoginPage = computed(() => route.path === '/login')
</script>

<template>
  <div class="app-container" :class="{ 'login-page': isLoginPage }">
    <aside class="sidebar" v-if="!isLoginPage">
      <div class="profile">
        <div class="avatar">
          <img src="https://api.dicebear.com/7.x/notionists/svg?seed=Felix" alt="头像">
        </div>
        <div class="profile-info">
          <h2>小雨</h2>
          <p>Living young and wild and free.</p>
        </div>
      </div>
      <nav class="menu">
        <router-link to="/" class="menu-item">
          <i class="fas fa-home"></i>
          首页
        </router-link>
        <router-link to="/friends" class="menu-item">
          <i class="fas fa-user-friends"></i>
          朋友
        </router-link>
        <router-link to="/photos" class="menu-item">
          <i class="fas fa-images"></i>
          相册
        </router-link>
        <router-link to="/diary" class="menu-item">
          <i class="fas fa-book"></i>
          日记
        </router-link>
        <router-link to="/about" class="menu-item">
          <i class="fas fa-info-circle"></i>
          关于
        </router-link>
      </nav>
    </aside>
    <main class="main-content" :class="{ 'full-width': isLoginPage }">
      <RouterView />
    </main>
    <aside class="right-sidebar" v-if="!isLoginPage">
      <div class="widget">
        <!-- 右侧边栏内容 -->
      </div>
    </aside>
  </div>
</template>

<style>
:root {
  --primary-color: #4a5568;
  --bg-color: #f1f5f9;
  --sidebar-bg: #ffffff;
  --text-color: #2d3748;
  --border-color: #e2e8f0;
}

body {
  margin: 0;
  padding: 0;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
  background-color: var(--bg-color);
  color: var(--text-color);
  width: 100%;
  min-height: 100vh;
  overflow-x: hidden;
}

#app {
  width: 100vw;
  min-height: 100vh;
  display: flex;
}

.app-container {
  display: grid;
  grid-template-columns: 240px minmax(600px, 1fr) 300px;
  min-height: 100vh;
  width: 100%;
  gap: 1rem;
  padding: 1rem;
  box-sizing: border-box;
}

.app-container.login-page {
  display: block;
  padding: 0;
}

.main-content.full-width {
  margin: 0;
  padding: 0;
  width: 100%;
  max-width: none;
}

.sidebar {
  background-color: var(--sidebar-bg);
  border-radius: 1rem;
  padding: 1.25rem;
  position: sticky;
  top: 1rem;
  height: calc(100vh - 2rem);
  box-shadow: 0 1px 3px rgba(0,0,0,0.1);
}

.profile {
  text-align: center;
  margin-bottom: 2rem;
}

.avatar img {
  width: 80px;
  height: 80px;
  border-radius: 50%;
  margin-bottom: 1rem;
}

.profile-info h2 {
  margin: 0;
  font-size: 1.25rem;
  color: var(--text-color);
}

.profile-info p {
  margin: 0.5rem 0;
  color: #718096;
  font-size: 0.9rem;
}

.menu {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.menu-item {
  display: flex;
  align-items: center;
  padding: 0.75rem 0.875rem;
  color: var(--text-color);
  text-decoration: none;
  border-radius: 0.5rem;
  transition: background-color 0.2s;
  font-size: 0.875rem;
}

.menu-item:hover {
  background-color: #f1f5f9;
}

.menu-item i {
  margin-right: 0.75rem;
  width: 20px;
}

.main-content {
  background-color: var(--sidebar-bg);
  border-radius: 1rem;
  padding: 1.5rem;
  box-shadow: 0 1px 3px rgba(0,0,0,0.1);
  display: flex;
  flex-direction: column;
  min-height: calc(100vh - 2rem);
  width: 100%;
  box-sizing: border-box;
}

.right-sidebar {
  background-color: var(--sidebar-bg);
  border-radius: 1rem;
  padding: 1.5rem;
  position: sticky;
  top: 1rem;
  height: calc(100vh - 2rem);
  box-shadow: 0 1px 3px rgba(0,0,0,0.1);
}

@media (max-width: 1280px) {
  .app-container {
    grid-template-columns: 240px minmax(0, 1fr);
    gap: 1rem;
  }
  
  .right-sidebar {
    display: none;
  }
}

@media (max-width: 768px) {
  .app-container {
    grid-template-columns: 1fr;
    padding: 0.5rem;
    gap: 0.5rem;
  }
  
  .sidebar {
    display: none;
  }

  .main-content {
    min-height: calc(100vh - 1rem);
  }
}
</style>
