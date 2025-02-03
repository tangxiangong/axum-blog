<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '@/stores/auth'
import {
  Menu as IconMenu,
  Document,
  Location,
  Setting,
  HomeFilled,
  CollectionTag
} from '@element-plus/icons-vue'

const router = useRouter()
const authStore = useAuthStore()

const isCollapse = ref(false)

const handleLogout = () => {
  authStore.logout()
  router.push('/login')
}
</script>

<template>
  <div class="min-h-screen">
    <el-container class="h-screen">
      <!-- 侧边栏 -->
      <el-aside :width="isCollapse ? '64px' : '200px'" class="bg-gray-800">
        <el-menu
          :collapse="isCollapse"
          class="h-full"
          background-color="#1F2937"
          text-color="#fff"
          active-text-color="#409EFF"
        >
          <el-menu-item index="1" @click="router.push('/admin')">
            <el-icon><HomeFilled /></el-icon>
            <span>仪表盘</span>
          </el-menu-item>
          
          <el-menu-item index="2" @click="router.push('/admin/posts')">
            <el-icon><Document /></el-icon>
            <span>文章管理</span>
          </el-menu-item>
          
          <el-menu-item index="3" @click="router.push('/admin/categories')">
            <el-icon><Location /></el-icon>
            <span>分类管理</span>
          </el-menu-item>
          
          <el-menu-item index="4" @click="router.push('/admin/tags')">
            <el-icon><CollectionTag /></el-icon>
            <span>标签管理</span>
          </el-menu-item>
          
          <el-menu-item index="5" @click="router.push('/admin/profile')">
            <el-icon><Setting /></el-icon>
            <span>个人设置</span>
          </el-menu-item>
        </el-menu>
      </el-aside>

      <!-- 主要内容区 -->
      <el-container>
        <el-header class="bg-white border-b flex items-center justify-between px-4">
          <el-button type="text" @click="isCollapse = !isCollapse">
            <el-icon><IconMenu /></el-icon>
          </el-button>
          
          <div class="flex items-center gap-4">
            <el-dropdown @command="handleLogout">
              <span class="flex items-center cursor-pointer">
                <el-avatar :size="32" :src="authStore.userInfo?.avatar" />
                <span class="ml-2">{{ authStore.userInfo?.username }}</span>
              </span>
              <template #dropdown>
                <el-dropdown-menu>
                  <el-dropdown-item command="logout">退出登录</el-dropdown-item>
                </el-dropdown-menu>
              </template>
            </el-dropdown>
          </div>
        </el-header>

        <el-main class="bg-gray-50">
          <router-view />
        </el-main>
      </el-container>
    </el-container>
  </div>
</template>

<style scoped>
.el-aside {
  transition: width 0.3s;
}

.el-header {
  height: 60px;
}
</style> 