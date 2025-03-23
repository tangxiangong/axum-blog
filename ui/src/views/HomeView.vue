<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import type { TabsPaneContext } from 'element-plus'
import PostList from '@/components/PostList.vue'

const router = useRouter()
const posts = ref([
  {
    id: 1,
    title: '真相不会因我们的承受能力而改变（2024年终总结）',
    summary: '这是我的第九份年终报告，回过头来发现已经走了很远很远了。写年终报告是一件没那么容易的事情，通常需要几天时间来整理思绪...',
    cover: 'https://picsum.photos/800/400?random=1',
    createdAt: '2024-01-30',
    author: '小雨',
    commentCount: 0
  },
  {
    id: 2,
    title: 'hello, 26岁！',
    summary: '当我用年龄计算器计算年龄的时候，计算器告诉我的人生已经度过33%。',
    cover: 'https://picsum.photos/800/400?random=2',
    createdAt: '2024-08-25',
    author: '小雨',
    commentCount: 32
  },
  {
    id: 3,
    title: '博客示例标题',
    summary: '这是文章摘要...',
    date: '2024-02-20',
    tags: ['Vue', 'TypeScript'],
  },
  {
    id: 4,
    title: '博客示例标题',
    summary: '这是文章摘要...',
    date: '2024-02-20',
    tags: ['Vue', 'TypeScript'],
  },
  // ... 更多文章
])

const activeTab = ref('latest')
const handleTabChange = (tab: TabsPaneContext) => {
  console.log(tab)
}
</script>

<template>
  <div class="home">
    <h1 class="page-title">最新文章</h1>
    <PostList :posts="posts" />
  </div>
</template>

<style scoped>
.home {
  width: 100%;
  display: flex;
  flex-direction: column;
  box-sizing: border-box;
}

.page-title {
  margin-bottom: 24px;
  font-size: 24px;
  color: #333;
}

/* 优化滚动条样式 */
::-webkit-scrollbar {
  width: 6px;
  height: 6px;
}

::-webkit-scrollbar-thumb {
  background: #d1d5db;
  border-radius: 3px;
}

::-webkit-scrollbar-track {
  background: #f3f4f6;
}

/* 调整卡片样式 */
.el-card {
  margin-bottom: 1.5rem;
  --el-card-padding: 20px;
}

.el-card :deep(.el-card__header) {
  padding: var(--el-card-padding);
  border-bottom: 1px solid #f0f0f0;
}

.el-card :deep(.el-card__body) {
  padding: var(--el-card-padding);
}
</style>
