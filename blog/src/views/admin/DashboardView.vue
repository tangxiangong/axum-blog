<script setup lang="ts">
import { ref, onMounted } from 'vue'

interface DashboardStats {
  totalPosts: number
  totalCategories: number
  totalTags: number
  recentPosts: Array<{
    id: number
    title: string
    createdAt: string
  }>
}

const stats = ref<DashboardStats>({
  totalPosts: 0,
  totalCategories: 0,
  totalTags: 0,
  recentPosts: []
})

onMounted(async () => {
  // TODO: 从API获取实际数据
  stats.value = {
    totalPosts: 25,
    totalCategories: 8,
    totalTags: 15,
    recentPosts: [
      { id: 1, title: '最新文章1', createdAt: '2024-03-10' },
      { id: 2, title: '最新文章2', createdAt: '2024-03-09' },
      { id: 3, title: '最新文章3', createdAt: '2024-03-08' },
    ]
  }
})
</script>

<template>
  <div class="p-6">
    <h1 class="text-2xl font-bold mb-6">仪表盘</h1>
    
    <!-- 统计卡片 -->
    <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
      <el-card shadow="hover">
        <div class="text-center">
          <h3 class="text-lg text-gray-600">文章总数</h3>
          <p class="text-3xl font-bold mt-2">{{ stats.totalPosts }}</p>
        </div>
      </el-card>
      
      <el-card shadow="hover">
        <div class="text-center">
          <h3 class="text-lg text-gray-600">分类总数</h3>
          <p class="text-3xl font-bold mt-2">{{ stats.totalCategories }}</p>
        </div>
      </el-card>
      
      <el-card shadow="hover">
        <div class="text-center">
          <h3 class="text-lg text-gray-600">标签总数</h3>
          <p class="text-3xl font-bold mt-2">{{ stats.totalTags }}</p>
        </div>
      </el-card>
    </div>
    
    <!-- 最近文章 -->
    <el-card class="w-full">
      <template #header>
        <div class="flex justify-between items-center">
          <span>最近发布的文章</span>
          <el-button type="primary" @click="$router.push('/admin/posts/create')">
            写文章
          </el-button>
        </div>
      </template>
      
      <el-table :data="stats.recentPosts" style="width: 100%">
        <el-table-column prop="title" label="标题" />
        <el-table-column prop="createdAt" label="发布时间" width="180" />
        <el-table-column fixed="right" label="操作" width="120">
          <template #default="{ row }">
            <el-button
              link
              type="primary"
              @click="$router.push(`/admin/posts/${row.id}/edit`)"
            >
              编辑
            </el-button>
          </template>
        </el-table-column>
      </el-table>
    </el-card>
  </div>
</template> 