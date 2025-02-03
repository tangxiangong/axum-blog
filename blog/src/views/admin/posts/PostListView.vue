 <script setup lang="ts">
import { ref, onMounted } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { Search } from '@element-plus/icons-vue'

interface Post {
  id: number
  title: string
  summary: string
  status: 'draft' | 'published'
  category: string
  tags: string[]
  createdAt: string
  updatedAt: string
}

const posts = ref<Post[]>([])
const loading = ref(false)
const search = ref('')

const fetchPosts = async () => {
  loading.value = true
  try {
    // TODO: 实现实际的API调用
    posts.value = [
      {
        id: 1,
        title: '示例文章1',
        summary: '这是一篇示例文章...',
        status: 'published',
        category: '技术',
        tags: ['Vue', 'TypeScript'],
        createdAt: '2024-03-10',
        updatedAt: '2024-03-10'
      },
      {
        id: 2,
        title: '示例文章2',
        summary: '这是另一篇示例文章...',
        status: 'draft',
        category: '随笔',
        tags: ['生活'],
        createdAt: '2024-03-09',
        updatedAt: '2024-03-09'
      }
    ]
  } catch (error) {
    ElMessage.error('获取文章列表失败')
  } finally {
    loading.value = false
  }
}

const handleDelete = async (id: number) => {
  try {
    await ElMessageBox.confirm('确定要删除这篇文章吗？', '提示', {
      type: 'warning'
    })
    // TODO: 实现实际的删除API调用
    posts.value = posts.value.filter(post => post.id !== id)
    ElMessage.success('删除成功')
  } catch {
    // 用户取消删除
  }
}

onMounted(() => {
  fetchPosts()
})
</script>

<template>
  <div class="p-6">
    <div class="flex justify-between items-center mb-6">
      <h1 class="text-2xl font-bold">文章管理</h1>
      <el-button type="primary" @click="$router.push('/admin/posts/create')">
        写文章
      </el-button>
    </div>

    <el-card>
      <div class="mb-4">
        <el-input
          v-model="search"
          placeholder="搜索文章标题..."
          class="w-80"
          clearable
        >
          <template #prefix>
            <el-icon><Search /></el-icon>
          </template>
        </el-input>
      </div>

      <el-table
        v-loading="loading"
        :data="posts.filter(post => post.title.includes(search))"
        style="width: 100%"
      >
        <el-table-column prop="title" label="标题" min-width="200" show-overflow-tooltip />
        <el-table-column prop="category" label="分类" width="120" />
        <el-table-column label="标签" width="200">
          <template #default="{ row }">
            <el-tag
              v-for="tag in row.tags"
              :key="tag"
              class="mr-1"
              size="small"
            >
              {{ tag }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="status" label="状态" width="100">
          <template #default="{ row }">
            <el-tag :type="row.status === 'published' ? 'success' : 'info'">
              {{ row.status === 'published' ? '已发布' : '草稿' }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="updatedAt" label="更新时间" width="180" />
        <el-table-column fixed="right" label="操作" width="150">
          <template #default="{ row }">
            <el-button
              link
              type="primary"
              @click="$router.push(`/admin/posts/${row.id}/edit`)"
            >
              编辑
            </el-button>
            <el-button
              link
              type="danger"
              @click="handleDelete(row.id)"
            >
              删除
            </el-button>
          </template>
        </el-table-column>
      </el-table>
    </el-card>
  </div>
</template>