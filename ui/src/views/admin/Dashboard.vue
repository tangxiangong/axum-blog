<script setup lang="ts">
import { ref } from 'vue'
import { adminApi, websiteApi } from '@/api'
import type { AdminInfo, WebsiteInfo } from '@/api/types'

const adminInfo = ref<AdminInfo>()
const websiteInfo = ref<WebsiteInfo>()
const loading = ref(false)

// 获取数据
const fetchData = async () => {
  try {
    loading.value = true
    const [adminResponse, websiteResponse] = await Promise.all([
      adminApi.getInfo(),
      websiteApi.getInfo()
    ])
    adminInfo.value = adminResponse.data
    websiteInfo.value = websiteResponse.data
  } catch (error) {
    console.error('获取数据失败:', error)
  } finally {
    loading.value = false
  }
}

// 组件挂载时获取数据
fetchData()
</script>

<template>
  <div class="p-6">
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
      <!-- 欢迎卡片 -->
      <el-card v-loading="loading">
        <template #header>
          <div class="flex items-center space-x-2">
            <el-icon><User /></el-icon>
            <span>欢迎回来</span>
          </div>
        </template>
        <template v-if="adminInfo">
          <div class="flex items-center space-x-4">
            <el-avatar :size="48" :src="adminInfo.avatar" />
            <div>
              <div class="font-bold">{{ adminInfo.nickname }}</div>
              <div class="text-gray-500 text-sm">{{ adminInfo.username }}</div>
            </div>
          </div>
        </template>
      </el-card>

      <!-- 网站信息卡片 -->
      <el-card v-loading="loading">
        <template #header>
          <div class="flex items-center space-x-2">
            <el-icon><Setting /></el-icon>
            <span>网站信息</span>
          </div>
        </template>
        <template v-if="websiteInfo">
          <div class="space-y-2">
            <div class="font-bold">{{ websiteInfo.title }}</div>
            <div class="text-gray-500 text-sm">{{ websiteInfo.subtitle }}</div>
          </div>
        </template>
      </el-card>

      <!-- 占位卡片 -->
      <el-card>
        <template #header>
          <div class="flex items-center space-x-2">
            <el-icon><Document /></el-icon>
            <span>文章统计</span>
          </div>
        </template>
        <div class="text-center text-2xl font-bold">
          0
        </div>
      </el-card>

      <!-- 占位卡片 -->
      <el-card>
        <template #header>
          <div class="flex items-center space-x-2">
            <el-icon><View /></el-icon>
            <span>访问统计</span>
          </div>
        </template>
        <div class="text-center text-2xl font-bold">
          0
        </div>
      </el-card>
    </div>
  </div>
</template> 