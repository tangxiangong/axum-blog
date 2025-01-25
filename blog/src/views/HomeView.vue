<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import type { TabsPaneContext } from 'element-plus'
import { HomeFilled, CollectionTag, Folder } from '@element-plus/icons-vue'

const router = useRouter()
const posts = ref([
  {
    id: 1,
    title: '博客示例标题',
    summary: '这是文章摘要...',
    date: '2024-02-20',
    tags: ['Vue', 'TypeScript'],
  },
  {
    id: 2,
    title: '博客示例标题',
    summary: '这是文章摘要...',
    date: '2024-02-20',
    tags: ['Vue', 'TypeScript'],
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
  <!-- 使用absolute布局实现铺满 -->
  <div class="absolute inset-0 flex flex-col bg-gray-50">
    <!-- 头部区域 -->
    <header class="bg-white shadow-sm flex-none">
      <div class="max-w-[1920px] h-[100px] mx-auto px-8 flex items-center">
        <div>
          <h1 class="text-4xl font-bold text-gray-800">我的博客</h1>
          <p class="text-gray-600 mt-2">分享技术，记录生活</p>
        </div>
      </div>
    </header>

    <!-- 内容区域 -->
    <div class="flex-1 overflow-hidden">
      <div class="max-w-[1920px] h-full mx-auto px-8">
        <main class="flex h-full gap-8 py-6">
          <!-- 左侧边栏 -->
          <aside class="lg:w-1/5 h-full overflow-y-auto pr-4">
            <div class="space-y-6">
              <!-- 导航菜单卡片 -->
              <el-card
                class="!border-0 shadow-sm hover:shadow-md transition-shadow rounded-xl nav-card"
              >
                <template #header>
                  <div class="text-lg font-semibold flex items-center gap-2">
                    <el-icon><HomeFilled /></el-icon>
                    导航
                  </div>
                </template>
                <nav class="flex flex-col -mx-3">
                  <a
                    v-for="item in navItems"
                    :key="item.path"
                    @click="handleNav(item.path)"
                    class="flex items-center gap-3 px-6 py-3 text-gray-600 hover:text-blue-600 hover:bg-blue-50 rounded-lg transition-colors cursor-pointer"
                  >
                    <el-icon><component :is="item.icon" /></el-icon>
                    <span>{{ item.name }}</span>
                  </a>
                </nav>
              </el-card>

              <!-- 关于我卡片 -->
              <el-card
                class="!border-0 shadow-sm hover:shadow-md transition-shadow rounded-xl about-card"
              >
                <template #header>
                  <div class="text-lg font-semibold border-b pb-3">关于我</div>
                </template>
                <div class="space-y-6">
                  <div class="flex flex-col items-center text-center gap-4">
                    <el-avatar
                      :size="80"
                      src="https://placeholder.co/100"
                      class="ring-4 ring-blue-50"
                    />
                    <div>
                      <h3 class="text-xl font-semibold text-gray-800">博主昵称</h3>
                      <p class="text-gray-500 mt-1">前端开发工程师</p>
                    </div>
                  </div>
                  <div class="flex justify-center gap-6 py-4 border-t border-b">
                    <div class="text-center">
                      <div class="text-xl font-semibold text-gray-800">52</div>
                      <div class="text-sm text-gray-500">文章</div>
                    </div>
                    <div class="text-center">
                      <div class="text-xl font-semibold text-gray-800">23</div>
                      <div class="text-sm text-gray-500">标签</div>
                    </div>
                    <div class="text-center">
                      <div class="text-xl font-semibold text-gray-800">12</div>
                      <div class="text-sm text-gray-500">分类</div>
                    </div>
                  </div>
                  <p class="text-gray-600 leading-relaxed">
                    热爱技术，热爱生活。分享Web开发相关的技术文章和学习心得。
                  </p>
                </div>
              </el-card>

              <!-- 标签云卡片 -->
              <el-card
                class="!border-0 shadow-sm hover:shadow-md transition-shadow rounded-xl tag-card"
              >
                <template #header>
                  <div class="text-lg font-semibold flex items-center gap-2">
                    <el-icon><CollectionTag /></el-icon>
                    标签云
                  </div>
                </template>
                <div class="flex flex-wrap gap-2">
                  <el-tag
                    v-for="tag in ['Vue', 'TypeScript', 'Element Plus', 'TailwindCSS']"
                    :key="tag"
                    class="cursor-pointer px-3 py-1"
                    effect="light"
                    round
                  >
                    {{ tag }}
                  </el-tag>
                </div>
              </el-card>
            </div>
          </aside>

          <!-- 文章列表区域 -->
          <div class="lg:w-4/5 h-full overflow-y-auto pr-4">
            <div class="space-y-6">
              <div
                v-for="post in posts"
                :key="post.id"
                class="bg-white p-6 rounded-lg shadow-sm hover:shadow-md transition-all"
              >
                <h2
                  class="text-2xl font-semibold mb-3 text-gray-800 hover:text-blue-600 cursor-pointer"
                >
                  {{ post.title }}
                </h2>
                <p class="text-gray-600 mb-4">{{ post.summary }}</p>
                <div class="flex justify-between items-center">
                  <div class="space-x-2">
                    <el-tag v-for="tag in post.tags" :key="tag" size="small" effect="light">
                      {{ tag }}
                    </el-tag>
                  </div>
                  <span class="text-gray-500">{{ post.date }}</span>
                </div>
              </div>
            </div>
          </div>
        </main>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* 优化滚动条样式 */
.overflow-y-auto {
  scrollbar-width: thin;
  scrollbar-color: #d1d5db #f3f4f6;
}

.overflow-y-auto::-webkit-scrollbar {
  width: 6px;
}

.overflow-y-auto::-webkit-scrollbar-track {
  background: #f3f4f6;
}

.overflow-y-auto::-webkit-scrollbar-thumb {
  background-color: #d1d5db;
  border-radius: 3px;
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

/* 添加滚动条样式 */
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
