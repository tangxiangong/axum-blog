<script setup lang="ts">
import { ref, reactive, onMounted, computed, watch } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { ElMessage, ElMessageBox } from 'element-plus'
import { articleApi, categoryApi, tagApi } from '@/api'
import type { 
  Article, 
  Category, 
  Tag, 
  CreateArticleRequest,
  UpdateArticleRequest
} from '@/api/types'

// 引入富文本编辑器组件
// 这里假设使用Element Plus的富文本编辑器
// 如果需要其他编辑器，如TinyMCE, CKEditor等，需要额外安装并引入
// import Editor from '@/components/Editor.vue' // 自定义编辑器组件

const router = useRouter()
const route = useRoute()
const loading = ref(false)
const submitting = ref(false)
const categories = ref<Category[]>([])
const tags = ref<Tag[]>([])

// 判断是否为编辑模式
const isEditMode = computed(() => Boolean(route.params.id))
const articleId = computed(() => Number(route.params.id) || 0)

// 文章表单
const articleForm = reactive<CreateArticleRequest>({
  title: '',
  summary: '',
  content: '',
  category_id: undefined,
  tag_ids: [],
  published: false
})

// 表单验证规则
const rules = {
  title: [
    { required: true, message: '请输入文章标题', trigger: 'blur' },
    { min: 3, max: 100, message: '标题长度应为3-100个字符', trigger: 'blur' }
  ],
  content: [
    { required: true, message: '请输入文章内容', trigger: 'blur' }
  ],
  category_id: [
    { required: true, message: '请选择文章分类', trigger: 'change' }
  ]
}

// 表单引用
const formRef = ref()

// 获取文章详情
const fetchArticle = async (id: number) => {
  try {
    loading.value = true
    const response = await articleApi.getArticle(id)
    const article = response.data
    
    // 更新表单数据
    Object.keys(articleForm).forEach(key => {
      if (key in article) {
        // @ts-ignore: 动态赋值
        articleForm[key] = article[key]
      }
    })
  } catch (error: any) {
    console.error('获取文章详情失败:', error)
    const errorMessage = error.response?.data?.message || '获取文章详情失败'
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

// 保存文章
const handleSave = async (publishNow = false) => {
  if (!formRef.value) return
  
  // 如果要发布，设置published为true
  if (publishNow) {
    articleForm.published = true
  }
  
  await formRef.value.validate(async (valid: boolean) => {
    if (valid) {
      try {
        submitting.value = true
        
        if (isEditMode.value) {
          // 更新文章
          await articleApi.updateArticle(articleId.value, articleForm as UpdateArticleRequest)
          ElMessage.success('文章更新成功')
        } else {
          // 创建文章
          await articleApi.createArticle(articleForm)
          ElMessage.success('文章创建成功')
        }
        
        // 返回文章列表页
        router.push('/admin/articles')
      } catch (error: any) {
        console.error('保存文章失败:', error)
        // 显示详细错误信息
        const errorMessage = error.response?.data?.message
        
        if (errorMessage) {
          // 根据错误信息类型提供更友好的提示
          if (errorMessage.includes('标题')) {
            ElMessage.error(`文章标题错误: ${errorMessage}`)
          } else if (errorMessage.includes('分类')) {
            ElMessage.error(`分类错误: ${errorMessage}`)
          } else if (errorMessage.includes('标签')) {
            ElMessage.error(`标签错误: ${errorMessage}`)
          } else if (errorMessage.includes('内容')) {
            ElMessage.error(`文章内容错误: ${errorMessage}`)
          } else {
            ElMessage.error(errorMessage)
          }
        } else {
          ElMessage.error('保存文章失败，请稍后重试')
        }
      } finally {
        submitting.value = false
      }
    } else {
      ElMessage.warning('请完善文章信息')
    }
  })
}

// 保存为草稿
const handleSaveAsDraft = async () => {
  articleForm.published = false
  await handleSave()
}

// 保存并发布
const handleSaveAndPublish = async () => {
  await handleSave(true)
}

// 取消编辑
const handleCancel = async () => {
  try {
    await ElMessageBox.confirm('确定要取消编辑吗？未保存的内容将丢失', '提示', {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning'
    })
    router.push('/admin/articles')
  } catch (error) {
    // 用户取消操作，不做任何处理
  }
}

// 生成页面标题
const pageTitle = computed(() => isEditMode.value ? '编辑文章' : '写文章')

// 监听路由变化，以便在参数变化时重新加载数据
watch(
  () => route.params.id,
  (newId) => {
    if (newId) {
      fetchArticle(Number(newId))
    }
  }
)

onMounted(async () => {
  await Promise.all([
    fetchCategories(),
    fetchTags()
  ])
  
  if (isEditMode.value) {
    fetchArticle(articleId.value)
  }
})
</script>

<template>
  <div class="container">
    <div class="page-header flex justify-between items-center mb-6">
      <h1 class="text-2xl font-bold">{{ pageTitle }}</h1>
      <div class="flex gap-2">
        <el-button @click="handleCancel">取消</el-button>
        <el-button type="info" @click="handleSaveAsDraft" :loading="submitting">
          保存草稿
        </el-button>
        <el-button type="primary" @click="handleSaveAndPublish" :loading="submitting">
          {{ isEditMode ? '更新' : '发布' }}
        </el-button>
      </div>
    </div>
    
    <el-card v-loading="loading">
      <el-form
        ref="formRef"
        :model="articleForm"
        :rules="rules"
        label-position="top"
      >
        <el-form-item label="文章标题" prop="title">
          <el-input
            v-model="articleForm.title"
            placeholder="请输入文章标题"
            maxlength="100"
            show-word-limit
          />
        </el-form-item>
        
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <el-form-item label="分类" prop="category_id">
            <el-select
              v-model="articleForm.category_id"
              placeholder="请选择分类"
              class="w-full"
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
              v-model="articleForm.tag_ids"
              placeholder="请选择标签"
              multiple
              class="w-full"
            >
              <el-option
                v-for="item in tags"
                :key="item.id"
                :label="item.name"
                :value="item.id"
              />
            </el-select>
          </el-form-item>
        </div>
        
        <el-form-item label="摘要">
          <el-input
            v-model="articleForm.summary"
            type="textarea"
            placeholder="请输入文章摘要"
            :rows="4"
            maxlength="200"
            show-word-limit
          />
        </el-form-item>
        
        <el-form-item label="文章内容" prop="content">
          <!-- 若有自定义编辑器组件，可以使用：
          <Editor v-model="articleForm.content" /> -->
          <el-input
            v-model="articleForm.content"
            type="textarea"
            placeholder="请输入文章内容"
            :rows="15"
          />
        </el-form-item>
        
        <el-form-item>
          <el-checkbox v-model="articleForm.published">
            发布(勾选则立即发布，否则保存为草稿)
          </el-checkbox>
        </el-form-item>
      </el-form>
      
      <div class="flex justify-end gap-2 mt-4">
        <el-button @click="handleCancel">取消</el-button>
        <el-button type="info" @click="handleSaveAsDraft" :loading="submitting">
          保存草稿
        </el-button>
        <el-button type="primary" @click="handleSaveAndPublish" :loading="submitting">
          {{ isEditMode ? '更新' : '发布' }}
        </el-button>
      </div>
    </el-card>
  </div>
</template>

<style scoped>
.container {
  padding: 20px;
}
</style> 