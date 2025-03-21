<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { useAuthStore } from '@/stores/auth'

const router = useRouter()
const route = useRoute()
const authStore = useAuthStore()
const isSidebarCollapsed = ref(false)

const toggleSidebar = () => {
  isSidebarCollapsed.value = !isSidebarCollapsed.value
  localStorage.setItem('sidebarCollapsed', String(isSidebarCollapsed.value))
}

onMounted(() => {
  isSidebarCollapsed.value = localStorage.getItem('sidebarCollapsed') === 'true'
})

const handleLogout = () => {
  authStore.logout()
  router.push('/admin/login')
}

const menuItems = [
  { path: '/admin/dashboard', icon: 'fas fa-tachometer-alt', label: '仪表盘', badge: 0 },
  { path: '/admin/posts', icon: 'fas fa-file-alt', label: '文章管理', badge: 5 },
  { path: '/admin/categories', icon: 'fas fa-folder', label: '分类管理', badge: 0 },
  { path: '/admin/tags', icon: 'fas fa-tags', label: '标签管理', badge: 2 },
  { path: '/admin/profile', icon: 'fas fa-user', label: '个人资料', badge: 0 },
  { path: '/admin/website', icon: 'fas fa-cog', label: '网站设置', badge: 0 },
]

const currentYear = new Date().getFullYear()
</script>

<template>
  <div class="admin-layout" :class="{ 'collapsed': isSidebarCollapsed }">
    <aside class="admin-sidebar">
      <div class="sidebar-header">
        <div class="logo-container">
          <span class="logo-icon"><i class="fas fa-feather-alt"></i></span>
          <h1 class="logo-text">小雨博客</h1>
        </div>
        <button class="collapse-btn" @click="toggleSidebar">
          <i :class="isSidebarCollapsed ? 'fas fa-indent' : 'fas fa-outdent'"></i>
        </button>
      </div>
      
      <div class="sidebar-user">
        <img src="https://api.dicebear.com/7.x/notionists/svg?seed=Felix" alt="头像" class="user-avatar">
        <div class="user-info">
          <h3 class="user-name">小雨</h3>
          <span class="user-role">管理员</span>
        </div>
      </div>
      
      <nav class="sidebar-nav">
        <router-link
          v-for="item in menuItems"
          :key="item.path"
          :to="item.path"
          class="nav-item"
          :class="{ 'active': route.path.startsWith(item.path) }"
        >
          <i :class="item.icon"></i>
          <span class="nav-label">{{ item.label }}</span>
          <span v-if="item.badge > 0" class="nav-badge">{{ item.badge }}</span>
        </router-link>
      </nav>
      
      <div class="sidebar-footer">
        <button class="logout-btn" @click="handleLogout">
          <i class="fas fa-sign-out-alt"></i>
          <span>退出登录</span>
        </button>
        <p class="copyright">© {{ currentYear }} 小雨博客</p>
      </div>
    </aside>
    
    <div class="admin-main">
      <header class="admin-header">
        <div class="header-left">
          <button class="menu-toggle" @click="toggleSidebar">
            <i class="fas fa-bars"></i>
          </button>
          <h2 class="page-title">{{ route.meta.title || '管理后台' }}</h2>
        </div>
        
        <div class="header-right">
          <div class="header-actions">
            <button class="action-btn">
              <i class="fas fa-bell"></i>
              <span class="action-badge">3</span>
            </button>
            <button class="action-btn">
              <i class="fas fa-envelope"></i>
              <span class="action-badge">2</span>
            </button>
            <a href="/" target="_blank" class="action-btn view-site-btn">
              <i class="fas fa-external-link-alt"></i>
              <span>查看网站</span>
            </a>
          </div>
        </div>
      </header>
      
      <main class="admin-content">
        <div class="content-wrapper">
          <RouterView />
        </div>
      </main>
      
      <footer class="admin-footer">
        <p>小雨博客管理系统 v1.0.0 | © {{ currentYear }} 版权所有</p>
      </footer>
    </div>
  </div>
</template>

<style scoped>
:root {
  --sidebar-width: 250px;
  --sidebar-collapsed-width: 70px;
  --header-height: 60px;
  --footer-height: 50px;
  --primary-color: #3b82f6;
  --primary-dark: #2563eb;
  --success-color: #10b981;
  --danger-color: #ef4444;
  --warning-color: #f59e0b;
  --info-color: #6366f1;
  --sidebar-bg: #1e293b;
  --sidebar-hover: #283548;
  --sidebar-active: #3b82f6;
  --header-bg: #ffffff;
  --content-bg: #f1f5f9;
  --text-light: #f8fafc;
  --text-muted: #94a3b8;
  --text-dark: #334155;
  --border-color: #e2e8f0;
  --transition: all 0.3s ease;
  --shadow-sm: 0 1px 2px 0 rgba(0, 0, 0, 0.05);
  --shadow: 0 1px 3px 0 rgba(0, 0, 0, 0.1), 0 1px 2px 0 rgba(0, 0, 0, 0.06);
  --shadow-md: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
}

.admin-layout {
  min-height: 100vh;
  display: grid;
  grid-template-columns: var(--sidebar-width) 1fr;
  transition: var(--transition);
}

.admin-layout.collapsed {
  grid-template-columns: var(--sidebar-collapsed-width) 1fr;
}

/* 侧边栏样式 */
.admin-sidebar {
  background-color: var(--sidebar-bg);
  color: var(--text-light);
  height: 100vh;
  position: fixed;
  width: var(--sidebar-width);
  transition: var(--transition);
  display: flex;
  flex-direction: column;
  overflow: hidden;
  z-index: 100;
  box-shadow: var(--shadow-md);
}

.admin-layout.collapsed .admin-sidebar {
  width: var(--sidebar-collapsed-width);
}

.sidebar-header {
  height: var(--header-height);
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 1rem;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}

.logo-container {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  overflow: hidden;
}

.logo-icon {
  font-size: 1.5rem;
  color: var(--primary-color);
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.logo-text {
  font-size: 1.25rem;
  font-weight: 700;
  margin: 0;
  white-space: nowrap;
  transition: var(--transition);
}

.admin-layout.collapsed .logo-text {
  opacity: 0;
  width: 0;
}

.collapse-btn {
  background: transparent;
  border: none;
  color: var(--text-muted);
  cursor: pointer;
  display: flex;
  padding: 0.5rem;
  font-size: 1rem;
  border-radius: 0.375rem;
  transition: var(--transition);
}

.collapse-btn:hover {
  color: var(--text-light);
  background-color: rgba(255, 255, 255, 0.1);
}

.sidebar-user {
  padding: 1.5rem 1rem;
  display: flex;
  align-items: center;
  gap: 1rem;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}

.user-avatar {
  width: 40px;
  height: 40px;
  border-radius: 50%;
  flex-shrink: 0;
  border: 2px solid var(--primary-color);
}

.user-info {
  overflow: hidden;
  transition: var(--transition);
}

.admin-layout.collapsed .user-info {
  opacity: 0;
  width: 0;
}

.user-name {
  margin: 0;
  font-size: 0.9rem;
  font-weight: 600;
}

.user-role {
  font-size: 0.8rem;
  color: var(--text-muted);
}

.sidebar-nav {
  padding: 1rem 0.5rem;
  overflow-y: auto;
  flex: 1;
}

.nav-item {
  display: flex;
  align-items: center;
  padding: 0.75rem;
  color: var(--text-light);
  text-decoration: none;
  border-radius: 0.5rem;
  margin-bottom: 0.25rem;
  transition: var(--transition);
  position: relative;
}

.nav-item:hover {
  background-color: var(--sidebar-hover);
}

.nav-item.active {
  background-color: var(--sidebar-active);
}

.nav-item i {
  font-size: 1rem;
  width: 1.5rem;
  text-align: center;
  margin-right: 0.75rem;
  flex-shrink: 0;
}

.nav-label {
  flex: 1;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  transition: var(--transition);
}

.admin-layout.collapsed .nav-label {
  opacity: 0;
  width: 0;
}

.nav-badge {
  background-color: var(--danger-color);
  color: white;
  font-size: 0.7rem;
  border-radius: 0.75rem;
  padding: 0.125rem 0.5rem;
  font-weight: 600;
  transition: var(--transition);
}

.admin-layout.collapsed .nav-badge {
  position: absolute;
  top: 0.25rem;
  right: 0.25rem;
  padding: 0.125rem 0.25rem;
  min-width: 0.5rem;
  height: 0.5rem;
  display: flex;
  align-items: center;
  justify-content: center;
}

.sidebar-footer {
  padding: 1rem;
  border-top: 1px solid rgba(255, 255, 255, 0.1);
}

.logout-btn {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  width: 100%;
  padding: 0.75rem;
  background-color: rgba(239, 68, 68, 0.2);
  color: var(--text-light);
  border: none;
  border-radius: 0.5rem;
  cursor: pointer;
  font-size: 0.875rem;
  transition: var(--transition);
  margin-bottom: 1rem;
}

.logout-btn:hover {
  background-color: rgba(239, 68, 68, 0.3);
}

.logout-btn i {
  font-size: 1rem;
}

.admin-layout.collapsed .logout-btn span {
  display: none;
}

.copyright {
  color: var(--text-muted);
  font-size: 0.75rem;
  text-align: center;
  margin: 0;
  white-space: nowrap;
  overflow: hidden;
}

.admin-layout.collapsed .copyright {
  display: none;
}

/* 主内容区域样式 */
.admin-main {
  margin-left: var(--sidebar-width);
  transition: var(--transition);
  min-height: 100vh;
  display: flex;
  flex-direction: column;
  background-color: var(--content-bg);
}

.admin-layout.collapsed .admin-main {
  margin-left: var(--sidebar-collapsed-width);
}

.admin-header {
  height: var(--header-height);
  background-color: var(--header-bg);
  box-shadow: var(--shadow-sm);
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 1.5rem;
  position: sticky;
  top: 0;
  z-index: 99;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.menu-toggle {
  background: transparent;
  border: none;
  color: var(--text-dark);
  font-size: 1.25rem;
  cursor: pointer;
  padding: 0.5rem;
  border-radius: 0.375rem;
  display: none;
}

.menu-toggle:hover {
  background-color: var(--border-color);
}

.page-title {
  margin: 0;
  font-size: 1.25rem;
  font-weight: 600;
  color: var(--text-dark);
}

.header-right {
  display: flex;
  align-items: center;
}

.header-actions {
  display: flex;
  align-items: center;
  gap: 0.75rem;
}

.action-btn {
  background: transparent;
  border: none;
  color: var(--text-dark);
  font-size: 1rem;
  cursor: pointer;
  padding: 0.5rem;
  border-radius: 0.375rem;
  position: relative;
  transition: var(--transition);
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.action-btn:hover {
  background-color: rgba(0, 0, 0, 0.05);
}

.action-badge {
  position: absolute;
  top: 0;
  right: 0;
  background-color: var(--danger-color);
  color: white;
  font-size: 0.7rem;
  font-weight: 600;
  width: 1rem;
  height: 1rem;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
}

.view-site-btn {
  color: var(--primary-color);
  background-color: rgba(59, 130, 246, 0.1);
  padding: 0.5rem 1rem;
  margin-left: 0.75rem;
  text-decoration: none;
}

.view-site-btn:hover {
  background-color: rgba(59, 130, 246, 0.15);
}

.admin-content {
  flex: 1;
  padding: 1.5rem;
  overflow-y: auto;
}

.content-wrapper {
  max-width: 1400px;
  margin: 0 auto;
  background-color: var(--header-bg);
  border-radius: 0.75rem;
  padding: 1.5rem;
  box-shadow: var(--shadow);
  min-height: calc(100vh - var(--header-height) - var(--footer-height) - 3rem);
}

.admin-footer {
  background-color: var(--header-bg);
  color: var(--text-muted);
  text-align: center;
  padding: 1rem;
  font-size: 0.875rem;
  height: var(--footer-height);
  display: flex;
  align-items: center;
  justify-content: center;
  border-top: 1px solid var(--border-color);
}

/* 响应式设计 */
@media (max-width: 1024px) {
  .admin-layout {
    grid-template-columns: var(--sidebar-collapsed-width) 1fr;
  }

  .admin-sidebar {
    width: var(--sidebar-collapsed-width);
  }

  .logo-text,
  .user-info,
  .nav-label {
    opacity: 0;
    width: 0;
  }

  .admin-main {
    margin-left: var(--sidebar-collapsed-width);
  }

  .menu-toggle {
    display: flex;
  }
}

@media (max-width: 768px) {
  .admin-layout {
    grid-template-columns: 1fr;
  }

  .admin-sidebar {
    transform: translateX(-100%);
    box-shadow: var(--shadow-md);
    width: var(--sidebar-width);
  }

  .admin-layout.menu-open .admin-sidebar {
    transform: translateX(0);
  }

  .logo-text,
  .user-info,
  .nav-label {
    opacity: 1;
    width: auto;
  }

  .admin-main {
    margin-left: 0;
  }

  .admin-header {
    padding: 0 1rem;
  }

  .view-site-btn span {
    display: none;
  }
}
</style> 