<template>
  <div class="blog-layout">
    <header class="blog-header">
      <div class="container">
        <div class="logo">
          <h1>博客系统</h1>
        </div>
        <nav class="nav">
          <router-link to="/">首页</router-link>
          <router-link to="/categories">分类</router-link>
          <router-link to="/tags">标签</router-link>
          <router-link to="/about">关于</router-link>
          <router-link to="/login" class="login-btn">后台管理</router-link>
        </nav>
      </div>
    </header>
    
    <main class="main-content">
      <div class="container">
        <router-view />
      </div>
    </main>
    
    <footer class="blog-footer">
      <div class="container">
        <p>© 2024 博客系统. 保留所有权利.</p>
      </div>
    </footer>
  </div>
</template>

<script setup lang="ts">
// 可以添加任何需要的逻辑
</script>

<style scoped>
.blog-layout {
  display: flex;
  flex-direction: column;
  min-height: 100vh;
}

.container {
  max-width: 1200px;
  margin: 0 auto;
  padding: 0 20px;
}

.blog-header {
  background-color: #fff;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  padding: 20px 0;
}

.blog-header .container {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.logo h1 {
  margin: 0;
  font-size: 24px;
  color: #333;
}

.nav {
  display: flex;
  gap: 20px;
}

.nav a {
  text-decoration: none;
  color: #333;
  font-weight: 500;
  transition: color 0.3s;
}

.nav a:hover, .nav a.router-link-active {
  color: #409eff;
}

.login-btn {
  background-color: #409eff;
  color: white !important;
  padding: 8px 16px;
  border-radius: 4px;
}

.main-content {
  flex: 1;
  padding: 40px 0;
  background-color: #f5f7fa;
}

.blog-footer {
  background-color: #333;
  color: #fff;
  padding: 20px 0;
  text-align: center;
}
</style> 