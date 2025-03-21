<script setup lang="ts">
import { RouterView, useRoute } from 'vue-router'
import { computed, onMounted, ref } from 'vue'

const route = useRoute()
const isLoginPage = computed(() => route.path === '/login')
const isDarkMode = ref(false)

// 检查系统主题偏好
onMounted(() => {
  const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches
  isDarkMode.value = localStorage.getItem('darkMode') === 'true' || prefersDark
  applyTheme()
})

// 切换暗色/亮色模式
const toggleDarkMode = () => {
  isDarkMode.value = !isDarkMode.value
  localStorage.setItem('darkMode', String(isDarkMode.value))
  applyTheme()
}

// 应用主题
const applyTheme = () => {
  document.documentElement.classList.toggle('dark', isDarkMode.value)
}
</script>

<template>
  <div 
    class="app-container" 
    :class="{ 
      'login-page': isLoginPage,
      'dark-mode': isDarkMode 
    }"
  >
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
      <div class="theme-toggle">
        <button @click="toggleDarkMode" class="theme-btn">
          <i :class="isDarkMode ? 'fas fa-sun' : 'fas fa-moon'"></i>
          {{ isDarkMode ? '亮色' : '暗色' }}模式
        </button>
      </div>
    </aside>
    <main class="main-content" :class="{ 'full-width': isLoginPage }">
      <RouterView />
    </main>
    <aside class="right-sidebar" v-if="!isLoginPage">
      <div class="widget">
        <div class="widget-title">
          <i class="fas fa-bullhorn"></i>
          公告
        </div>
        <div class="widget-content">
          <p>欢迎来到我的个人博客！这里记录我的日常和思考。</p>
        </div>
      </div>
      <div class="widget">
        <div class="widget-title">
          <i class="fas fa-tags"></i>
          热门标签
        </div>
        <div class="widget-content tag-cloud">
          <span class="tag">生活</span>
          <span class="tag">技术</span>
          <span class="tag">旅行</span>
          <span class="tag">音乐</span>
          <span class="tag">随笔</span>
        </div>
      </div>
      <div class="widget">
        <div class="widget-title">
          <i class="fas fa-link"></i>
          友情链接
        </div>
        <div class="widget-content">
          <a href="#" class="friend-link">朋友的博客</a>
          <a href="#" class="friend-link">技术社区</a>
          <a href="#" class="friend-link">摄影网站</a>
        </div>
      </div>
    </aside>
  </div>
</template>

<style>
:root {
  --primary-color: #3b82f6;
  --primary-hover: #2563eb;
  --bg-color: #f9fafb;
  --bg-card: #ffffff;
  --text-color: #1f2937;
  --text-secondary: #6b7280;
  --border-color: #e5e7eb;
  --sidebar-hover: #f3f4f6;
  --header-bg: #ffffff;
  --box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1), 0 1px 2px rgba(0, 0, 0, 0.06);
  --transition: all 0.3s ease;
}

:root.dark {
  --primary-color: #3b82f6;
  --primary-hover: #60a5fa;
  --bg-color: #111827;
  --bg-card: #1f2937;
  --text-color: #f3f4f6;
  --text-secondary: #9ca3af;
  --border-color: #374151;
  --sidebar-hover: #374151;
  --header-bg: #1f2937;
  --box-shadow: 0 1px 3px rgba(0, 0, 0, 0.3);
}

body {
  margin: 0;
  padding: 0;
  font-family: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
  background-color: var(--bg-color);
  color: var(--text-color);
  width: 100%;
  min-height: 100vh;
  overflow-x: hidden;
  transition: var(--transition);
}

#app {
  width: 100vw;
  min-height: 100vh;
  display: flex;
}

.app-container {
  display: grid;
  grid-template-columns: 280px minmax(600px, 1fr) 320px;
  min-height: 100vh;
  width: 100%;
  gap: 1.5rem;
  padding: 1.5rem;
  box-sizing: border-box;
  transition: var(--transition);
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
  background-color: var(--bg-card);
  border-radius: 1rem;
  padding: 1.5rem;
  position: sticky;
  top: 1.5rem;
  height: calc(100vh - 3rem);
  box-shadow: var(--box-shadow);
  display: flex;
  flex-direction: column;
  transition: var(--transition);
}

.profile {
  text-align: center;
  margin-bottom: 2rem;
  padding-bottom: 1.5rem;
  border-bottom: 1px solid var(--border-color);
}

.avatar img {
  width: 100px;
  height: 100px;
  border-radius: 50%;
  margin-bottom: 1rem;
  box-shadow: var(--box-shadow);
  border: 3px solid var(--primary-color);
  padding: 3px;
  transition: transform 0.3s ease;
}

.avatar img:hover {
  transform: scale(1.05);
}

.profile-info h2 {
  margin: 0.5rem 0;
  font-size: 1.5rem;
  font-weight: 700;
  color: var(--text-color);
}

.profile-info p {
  margin: 0.5rem 0;
  color: var(--text-secondary);
  font-size: 0.95rem;
  font-style: italic;
}

.menu {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
  flex: 1;
}

.menu-item {
  display: flex;
  align-items: center;
  padding: 0.9rem 1rem;
  color: var(--text-color);
  text-decoration: none;
  border-radius: 0.75rem;
  transition: var(--transition);
  font-size: 1rem;
  font-weight: 500;
}

.menu-item:hover {
  background-color: var(--sidebar-hover);
  transform: translateX(5px);
}

.menu-item.router-link-active {
  background-color: var(--primary-color);
  color: white;
}

.menu-item i {
  margin-right: 1rem;
  width: 20px;
  text-align: center;
  font-size: 1rem;
}

.theme-toggle {
  margin-top: auto;
  padding-top: 1.5rem;
  border-top: 1px solid var(--border-color);
}

.theme-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 100%;
  padding: 0.9rem 1rem;
  background-color: var(--bg-color);
  color: var(--text-color);
  border: 1px solid var(--border-color);
  border-radius: 0.75rem;
  cursor: pointer;
  font-size: 0.95rem;
  font-weight: 500;
  transition: var(--transition);
}

.theme-btn:hover {
  background-color: var(--sidebar-hover);
}

.theme-btn i {
  margin-right: 0.5rem;
  font-size: 1rem;
}

.main-content {
  background-color: var(--bg-card);
  border-radius: 1rem;
  padding: 2rem;
  box-shadow: var(--box-shadow);
  display: flex;
  flex-direction: column;
  min-height: calc(100vh - 3rem);
  width: 100%;
  box-sizing: border-box;
  overflow: auto;
  transition: var(--transition);
}

.right-sidebar {
  background-color: var(--bg-card);
  border-radius: 1rem;
  padding: 1.5rem;
  position: sticky;
  top: 1.5rem;
  height: calc(100vh - 3rem);
  box-shadow: var(--box-shadow);
  overflow-y: auto;
  transition: var(--transition);
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.widget {
  background-color: var(--bg-color);
  border-radius: 0.75rem;
  overflow: hidden;
  transition: var(--transition);
  box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
}

.widget-title {
  padding: 1rem 1.25rem;
  font-weight: 600;
  font-size: 1rem;
  color: var(--text-color);
  border-bottom: 1px solid var(--border-color);
  display: flex;
  align-items: center;
}

.widget-title i {
  margin-right: 0.5rem;
  color: var(--primary-color);
}

.widget-content {
  padding: 1rem 1.25rem;
}

.tag-cloud {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
}

.tag {
  display: inline-block;
  padding: 0.3rem 0.6rem;
  background-color: var(--primary-color);
  color: white;
  border-radius: 0.375rem;
  font-size: 0.75rem;
  font-weight: 500;
  transition: var(--transition);
}

.tag:hover {
  background-color: var(--primary-hover);
  transform: translateY(-2px);
}

.friend-link {
  display: block;
  padding: 0.5rem 0;
  color: var(--text-color);
  text-decoration: none;
  border-bottom: 1px dashed var(--border-color);
  transition: var(--transition);
}

.friend-link:last-child {
  border-bottom: none;
}

.friend-link:hover {
  color: var(--primary-color);
  transform: translateX(5px);
}

/* 滚动条美化 */
::-webkit-scrollbar {
  width: 6px;
  height: 6px;
}

::-webkit-scrollbar-track {
  background: transparent;
}

::-webkit-scrollbar-thumb {
  background: var(--primary-color);
  border-radius: 3px;
}

/* 响应式设计 */
@media (max-width: 1280px) {
  .app-container {
    grid-template-columns: 250px minmax(0, 1fr);
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
    padding: 1rem;
  }
}
</style>
