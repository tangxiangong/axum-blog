<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '@/stores/auth'

const router = useRouter()
const authStore = useAuthStore()

const handleLogout = () => {
  authStore.logout()
  router.push('/admin/login')
}
</script>

<template>
  <div class="admin-layout">
    <header class="admin-header">
      <div class="header-left">
        <h1 class="site-title">博客后台</h1>
        <nav class="main-nav">
          <router-link to="/admin/dashboard" class="nav-item">
            <i class="fas fa-tachometer-alt"></i>
            仪表盘
          </router-link>
          <router-link to="/admin/posts" class="nav-item">
            <i class="fas fa-file-alt"></i>
            文章管理
          </router-link>
          <router-link to="/admin/categories" class="nav-item">
            <i class="fas fa-folder"></i>
            分类管理
          </router-link>
          <router-link to="/admin/tags" class="nav-item">
            <i class="fas fa-tags"></i>
            标签管理
          </router-link>
          <router-link to="/admin/profile" class="nav-item">
            <i class="fas fa-user"></i>
            个人资料
          </router-link>
        </nav>
      </div>
      <div class="header-right">
        <div class="user-info">
          <span>小雨</span>
          <img src="https://api.dicebear.com/7.x/notionists/svg?seed=Felix" alt="头像" class="avatar">
        </div>
        <button class="logout-btn" @click="handleLogout">
          <i class="fas fa-sign-out-alt"></i>
          退出
        </button>
      </div>
    </header>
    <main class="admin-content">
      <div class="content-wrapper">
        <RouterView />
      </div>
    </main>
  </div>
</template>

<style scoped>
.admin-layout {
  min-height: 100vh;
  background-color: #f1f5f9;
}

.admin-header {
  height: 64px;
  background-color: #1e293b;
  color: #e2e8f0;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 1.5rem;
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  z-index: 10;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 2rem;
}

.site-title {
  margin: 0;
  font-size: 1.25rem;
  font-weight: 600;
  color: white;
}

.main-nav {
  display: flex;
  gap: 1rem;
}

.nav-item {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  color: #e2e8f0;
  text-decoration: none;
  padding: 0.5rem 0.75rem;
  border-radius: 0.375rem;
  transition: background-color 0.2s;
  font-size: 0.875rem;
}

.nav-item:hover {
  background-color: #334155;
}

.nav-item.router-link-active {
  background-color: #2563eb;
  color: white;
}

.nav-item i {
  font-size: 0.875rem;
}

.header-right {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.user-info {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  color: white;
}

.avatar {
  width: 32px;
  height: 32px;
  border-radius: 50%;
}

.logout-btn {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  background-color: #dc2626;
  color: white;
  border: none;
  padding: 0.5rem 1rem;
  border-radius: 0.375rem;
  cursor: pointer;
  font-size: 0.875rem;
  transition: background-color 0.2s;
}

.logout-btn:hover {
  background-color: #b91c1c;
}

.admin-content {
  padding-top: 64px;
  min-height: calc(100vh - 64px);
}

.content-wrapper {
  padding: 1.5rem;
  max-width: 1200px;
  margin: 0 auto;
}
</style> 