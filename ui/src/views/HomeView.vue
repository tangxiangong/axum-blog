<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import type { TabsPaneContext } from 'element-plus'
import { HomeFilled, CollectionTag, Folder } from '@element-plus/icons-vue'
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

const navItems = [
  { name: '首页', path: '/', icon: HomeFilled },
  { name: '标签', path: '/tags', icon: CollectionTag },
  { name: '分类', path: '/categories', icon: Folder },
]

const handleNav = (path: string) => {
  router.push(path)
}

const activeTab = ref('latest')
const handleTabChange = (tab: TabsPaneContext) => {
  console.log(tab)
}
</script>

<template>
  <div class="home">
    <PostList :posts="posts" />
  </div>
</template>

<style scoped>
.home {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  box-sizing: border-box;
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
.el-tabs :deep(.el-tabs__content) {
  flex: 1;
  overflow: hidden;
}

.el-card {
  margin-bottom: 1.5rem;
  --el-card-padding: 20px;
}

.el-tabs :deep(.el-tabs__header) {
  margin-bottom: 24px;
}

.el-card :deep(.el-card__header) {
  padding: var(--el-card-padding);
  border-bottom: 1px solid #f0f0f0;
}

.el-card :deep(.el-card__body) {
  padding: var(--el-card-padding);
}

/* 确保内容区域滚动 */
.el-tabs :deep(.el-tab-pane) {
  height: 100%;
}

.about-card {
  height: auto !important;
}

.about-card :deep(.el-card__body) {
  height: auto !important;
  overflow: visible;
}

/* 导航菜单样式 */
.nav-card :deep(.el-card__body) {
  padding: 0.5rem 0;
}

/* 标签云卡片样式 */
.tag-card :deep(.el-card__body) {
  padding-top: 0.5rem;
  padding-bottom: 0.5rem;
}

/* 卡片间距控制 */
.space-y-6 > * {
  margin-bottom: 1.5rem;
}

.space-y-6 > *:last-child {
  margin-bottom: 0;
}
</style>
