<script setup lang="ts">
import { ref, reactive, onMounted, computed } from 'vue'
import { useRouter } from 'vue-router'
import { ElMessage, ElMessageBox } from 'element-plus'
import { Search, Edit, Delete, View } from '@element-plus/icons-vue'
import { articleApi, categoryApi, tagApi } from '@/api'
import type { 
  Article, 
  ArticleListItem, 
  ListArticlesRequest, 
  Category, 
  Tag,
  Pagination
} from '@/api/types'

const router = useRouter()
const loading = ref(false)
const articles = ref<ArticleListItem[]>([])
const categories = ref<Category[]>([])
const tags = ref<Tag[]>([])
const total = ref(0)

// 分页和查询参数
const queryParams = reactive<ListArticlesRequest>({
  page: 1,
  page_size: 10,
  title: '',
  category_id: undefined,
  tag_id: undefined,
  published: undefined
})

// 状态过滤选项
const publishStatusOptions = [
  { value: true, label: '已发布' },
  { value: false, label: '草稿' }
]

// 获取文章列表
const fetchArticles = async () => {
  try {
    loading.value = true
    const response = await articleApi.listArticles(queryParams)
    articles.value = response.data.items || []
    total.value = response.data.total || 0
  } catch (error: any) {
    console.error('获取文章失败:', error)
    const errorMessage = error.response?.data?.message || '获取文章列表失败'
    ElMessage.error(errorMessage)
  } finally {
    loading.value = false
  }
}

// 获取分类列表
const fetchCategories = async () => {
  try {
    const response = await categoryApi.listCategories()
    categories.value = response.data.items || []
  } catch (error: any) {
    console.error('获取分类失败:', error)
    const errorMessage = error.response?.data?.message || '获取分类列表失败'
    ElMessage.error(errorMessage)
  }
}

// 获取标签列表
const fetchTags = async () => {
  try {
    const response = await tagApi.listTags()
    tags.value = response.data.items || []
  } catch (error: any) {
    console.error('获取标签失败:', error)
    const errorMessage = error.response?.data?.message || '获取标签列表失败'
    ElMessage.error(errorMessage)
  }
}

// 处理搜索
const handleSearch = () => {
  queryParams.page = 1
  fetchArticles()
}

// 重置搜索条件
const resetSearch = () => {
  queryParams.title = ''
  queryParams.category_id = undefined
  queryParams.tag_id = undefined
  queryParams.published = undefined
  queryParams.page = 1
  fetchArticles()
}

// 处理分页变化
const handlePageChange = (page: number) => {
  queryParams.page = page
  fetchArticles()
}

// 处理每页显示数量变化
const handleSizeChange = (size: number) => {
  queryParams.page_size = size
  queryParams.page = 1
  fetchArticles()
}

// 新建文章
const handleCreate = () => {
  router.push('/admin/articles/create')
}

// 编辑文章
const handleEdit = (id: number) => {
  router.push(`/admin/articles/edit/${id}`)
}

// 查看文章
const handleView = (id: number) => {
  // 打开文章预览，可以在新窗口打开前台文章页面
  window.open(`/articles/${id}`, '_blank')
}

// 删除文章
const handleDelete = async (id: number) => {
  try {
    await ElMessageBox.confirm('确定要删除这篇文章吗？此操作不可恢复', '提示', {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning'
    })
    
    await articleApi.deleteArticle(id)
    ElMessage.success('删除成功')
    fetchArticles()
  } catch (error: any) {
    if (error !== 'cancel') {
      console.error('删除文章失败:', error)
      
      // 显示详细错误信息
      const errorMessage = error.response?.data?.message
      if (errorMessage) {
        ElMessage.error(`删除失败: ${errorMessage}`)
      } else {
        ElMessage.error('删除文章失败，请稍后重试')
      }
    }
  }
}

// 切换文章发布状态
const handleTogglePublish = async (row: ArticleListItem) => {
  const action = row.published ? '取消发布' : '发布'
  try {
    await ElMessageBox.confirm(`确定要${action}这篇文章吗？`, '提示', {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning'
    })
    
    await articleApi.updateArticle(row.id, {
      published: !row.published
    })
    
    ElMessage.success(`${action}成功`)
    fetchArticles()
  } catch (error: any) {
    if (error !== 'cancel') {
      console.error(`${action}文章失败:`, error)
      const errorMessage = error.response?.data?.message || `${action}文章失败，请稍后重试`
      ElMessage.error(errorMessage)
    }
  }
}

// 格式化时间
const formatDate = (dateString: string) => {
  if (!dateString) return '-'
  const date = new Date(dateString)
  return date.toLocaleString('zh-CN', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit'
  })
}

// 根据ID获取分类名称
const getCategoryName = (id: number | undefined) => {
  if (!id) return '-'
  const category = categories.value.find(c => c.id === id)
  return category ? category.name : '-'
}

// 根据IDs获取标签名称
const getTagNames = (tagIds: number[] | undefined) => {
  if (!tagIds || tagIds.length === 0) return '-'
  return tagIds
    .map(id => tags.value.find(t => t.id === id)?.name || '')
    .filter(Boolean)
    .join(', ')
}

onMounted(async () => {
  await Promise.all([
    fetchCategories(),
    fetchTags()
  ])
  fetchArticles()
})
</script>

<template>
  <div class="container">
    <div class="page-header flex justify-between items-center mb-6">
      <h1 class="text-2xl font-bold">文章管理</h1>
      <el-button type="primary" @click="handleCreate">写文章</el-button>
    </div>
    
    <el-card class="mb-6">
      <el-form :model="queryParams" inline>
        <el-form-item label="标题">
          <el-input
            v-model="queryParams.title"
            placeholder="请输入文章标题"
            clearable
            @keyup.enter="handleSearch"
          />
        </el-form-item>
        
        <el-form-item label="分类">
          <el-select
            v-model="queryParams.category_id"
            placeholder="请选择分类"
            clearable
          >
            <el-option
              v-for="item in categories"
              :key="item.id"
              :label="item.name"
              :value="item.id"
            />
          </el-select>
        </el-form-item>
        
        <el-form-item label="标签">
          <el-select
            v-model="queryParams.tag_id"
            placeholder="请选择标签"
            clearable
          >
            <el-option
              v-for="item in tags"
              :key="item.id"
              :label="item.name"
              :value="item.id"
            />
          </el-select>
        </el-form-item>
        
        <el-form-item label="状态">
          <el-select
            v-model="queryParams.published"
            placeholder="请选择状态"
            clearable
          >
            <el-option
              v-for="item in publishStatusOptions"
              :key="String(item.value)"
              :label="item.label"
              :value="item.value"
            />
          </el-select>
        </el-form-item>
        
        <el-form-item>
          <el-button type="primary" :icon="Search" @click="handleSearch">搜索</el-button>
          <el-button @click="resetSearch">重置</el-button>
        </el-form-item>
      </el-form>
    </el-card>
    
    <el-card>
      <el-table
        v-loading="loading"
        :data="articles"
        stripe
        border
        style="width: 100%"
      >
        <el-table-column prop="id" label="ID" width="80" />
        <el-table-column prop="title" label="标题" min-width="200" show-overflow-tooltip />
        <el-table-column label="分类" width="120">
          <template #default="{ row }">
            {{ getCategoryName(row.category_id) }}
          </template>
        </el-table-column>
        <el-table-column label="标签" width="180" show-overflow-tooltip>
          <template #default="{ row }">
            {{ getTagNames(row.tag_ids) }}
          </template>
        </el-table-column>
        <el-table-column label="状态" width="100">
          <template #default="{ row }">
            <el-tag :type="row.published ? 'success' : 'info'">
              {{ row.published ? '已发布' : '草稿' }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column label="创建时间" width="180">
          <template #default="{ row }">
            {{ formatDate(row.created_at) }}
          </template>
        </el-table-column>
        <el-table-column label="更新时间" width="180">
          <template #default="{ row }">
            {{ formatDate(row.updated_at) }}
          </template>
        </el-table-column>
        <el-table-column label="操作" width="220" fixed="right">
          <template #default="{ row }">
            <el-button size="small" type="primary" :icon="Edit" @click="handleEdit(row.id)">
              编辑
            </el-button>
            <el-button size="small" type="danger" :icon="Delete" @click="handleDelete(row.id)">
              删除
            </el-button>
            <el-button
              size="small"
              :type="row.published ? 'warning' : 'success'"
              @click="handleTogglePublish(row)"
            >
              {{ row.published ? '取消发布' : '发布' }}
            </el-button>
          </template>
        </el-table-column>
      </el-table>
      
      <div class="flex justify-center mt-4">
        <el-pagination
          v-model:current-page="queryParams.page"
          v-model:page-size="queryParams.page_size"
          :total="total"
          :page-sizes="[10, 20, 50, 100]"
          layout="total, sizes, prev, pager, next, jumper"
          @size-change="handleSizeChange"
          @current-change="handlePageChange"
        />
      </div>
    </el-card>
  </div>
</template>

<style scoped>
.container {
  padding: 20px;
}
</style> 