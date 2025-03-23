<script setup lang="ts">
import { ref, computed } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { ElMessageBox, ElMessage } from 'element-plus'
import { adminApi } from '@/api'
import type { AdminInfo } from '@/api/types'

const router = useRouter()
const route = useRoute()
const adminInfo = ref<AdminInfo>()

// 计算当前激活的菜单项
const activeMenu = computed(() => route.path)

const getAdminInfo = async () => {
  try {
    const { data } = await adminApi.getInfo()
    adminInfo.value = data
  } catch (error) {
    console.error('获取管理员信息失败:', error)
  }
}

const handleLogout = async () => {
  try {
    await ElMessageBox.confirm('确定要退出登录吗？', '提示', {
      type: 'warning'
    })
    await adminApi.signOut()
    router.push('/login')
    ElMessage.success('已退出登录')
  } catch (error) {
    if (error !== 'cancel') {
      console.error('退出登录失败:', error)
    }
  }
}

// 在组件挂载时获取管理员信息
getAdminInfo()
</script>

<template>
  <el-container class="min-h-screen">
    <el-aside width="200px" class="bg-gray-800">
      <div class="p-4">
        <h1 class="text-white text-xl font-bold">博客管理系统</h1>
      </div>
      <el-menu
        class="border-none"
        background-color="#1F2937"
        text-color="#fff"
        active-text-color="#409EFF"
        :default-active="activeMenu"
        router
      >
        <el-menu-item index="/admin/dashboard">
          <el-icon><DataLine /></el-icon>
          <span>仪表盘</span>
        </el-menu-item>
        
        <el-sub-menu index="/admin/content">
          <template #title>
            <el-icon><Document /></el-icon>
            <span>内容管理</span>
          </template>
          <el-menu-item index="/admin/articles">
            <el-icon><Reading /></el-icon>
            <span>文章管理</span>
          </el-menu-item>
          <el-menu-item index="/admin/categories">
            <el-icon><Files /></el-icon>
            <span>分类管理</span>
          </el-menu-item>
          <el-menu-item index="/admin/tags">
            <el-icon><Discount /></el-icon>
            <span>标签管理</span>
          </el-menu-item>
        </el-sub-menu>
        
        <el-menu-item index="/admin/website">
          <el-icon><Setting /></el-icon>
          <span>网站设置</span>
        </el-menu-item>
        
        <el-menu-item index="/admin/profile">
          <el-icon><User /></el-icon>
          <span>个人资料</span>
        </el-menu-item>
      </el-menu>
    </el-aside>

    <el-container>
      <el-header class="bg-white border-b flex items-center justify-between px-4">
        <div class="flex items-center">
          <el-breadcrumb>
            <el-breadcrumb-item>首页</el-breadcrumb-item>
            <el-breadcrumb-item>{{ $route.meta.title }}</el-breadcrumb-item>
          </el-breadcrumb>
        </div>
        <el-dropdown @command="handleLogout">
          <span class="flex items-center cursor-pointer">
            <el-avatar :size="32" :src="adminInfo?.avatar" />
            <span class="ml-2">{{ adminInfo?.nickname }}</span>
          </span>
          <template #dropdown>
            <el-dropdown-menu>
              <el-dropdown-item command="logout">退出登录</el-dropdown-item>
            </el-dropdown-menu>
          </template>
        </el-dropdown>
      </el-header>

      <el-main class="bg-gray-50">
        <router-view />
      </el-main>
    </el-container>
  </el-container>
</template> 