<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { ElMessage } from 'element-plus'
import type { FormInstance, FormRules } from 'element-plus'
import { MdEditor } from 'md-editor-v3'
import 'md-editor-v3/lib/style.css'

const route = useRoute()
const router = useRouter()
const isEdit = route.params.id !== undefined

interface PostForm {
  title: string
  content: string
  summary: string
  category: string
  tags: string[]
  status: 'draft' | 'published'
}

const postForm = ref<PostForm>({
  title: '',
  content: '',
  summary: '',
  category: '',
  tags: [],
  status: 'draft'
})

const formRef = ref<FormInstance>()
const loading = ref(false)
const categories = ref<string[]>(['技术', '随笔', '生活'])
const availableTags = ref<string[]>(['Vue', 'TypeScript', 'JavaScript', '前端'])

// 编辑器配置
const editorConfig = {
  toolbars: [
    'bold', 'underline', 'italic', 'strikethrough', 'title', 'sub', 'sup', 
    'quote', 'unordered-list', 'ordered-list', 'task-list', '-',
    'code', 'code-block', 'link', 'image', 'table', 'mermaid', 'katex', '-',
    'preview', 'fullscreen'
  ]
}

const handleUploadImage = async (files: FileList, callback: (urls: string[]) => void) => {
  try {
    // TODO: 实现实际的图片上传API调用
    const urls = await Promise.all(
      Array.from(files).map(async (file) => {
        // 模拟上传
        await new Promise(resolve => setTimeout(resolve, 1000))
        return URL.createObjectURL(file)
      })
    )
    callback(urls)
  } catch (error) {
    ElMessage.error('图片上传失败')
  }
}

const rules = ref<FormRules>({
  title: [
    { required: true, message: '请输入文章标题', trigger: 'blur' },
    { min: 2, max: 100, message: '标题长度应在2-100个字符之间', trigger: 'blur' }
  ],
  category: [
    { required: true, message: '请选择文章分类', trigger: 'change' }
  ],
  content: [
    { required: true, message: '请输入文章内容', trigger: 'blur' }
  ]
})

const fetchPost = async (id: string) => {
  loading.value = true
  try {
    // TODO: 实现实际的API调用
    postForm.value = {
      title: '示例文章',
      content: '# 示例文章\n\n这是文章内容...',
      summary: '这是文章摘要...',
      category: '技术',
      tags: ['Vue', 'TypeScript'],
      status: 'draft'
    }
  } catch (error) {
    ElMessage.error('获取文章失败')
  } finally {
    loading.value = false
  }
}

const handleSubmit = async (formEl: FormInstance | undefined) => {
  if (!formEl) return
  
  await formEl.validate(async (valid) => {
    if (valid) {
      loading.value = true
      try {
        // TODO: 实现实际的API调用
        await new Promise(resolve => setTimeout(resolve, 1000))
        ElMessage.success(isEdit ? '更新成功' : '创建成功')
        router.push('/admin/posts')
      } catch (error) {
        ElMessage.error(isEdit ? '更新失败' : '创建失败')
      } finally {
        loading.value = false
      }
    }
  })
}

onMounted(() => {
  if (isEdit) {
    fetchPost(route.params.id as string)
  }
})
</script>

<template>
  <div class="p-6">
    <div class="flex justify-between items-center mb-6">
      <h1 class="text-2xl font-bold">{{ isEdit ? '编辑文章' : '写文章' }}</h1>
    </div>

    <el-card v-loading="loading">
      <el-form
        ref="formRef"
        :model="postForm"
        :rules="rules"
        label-width="100px"
        status-icon
      >
        <el-form-item label="标题" prop="title">
          <el-input v-model="postForm.title" placeholder="请输入文章标题" />
        </el-form-item>

        <el-form-item label="分类" prop="category">
          <el-select v-model="postForm.category" placeholder="请选择分类">
            <el-option
              v-for="category in categories"
              :key="category"
              :label="category"
              :value="category"
            />
          </el-select>
        </el-form-item>

        <el-form-item label="标签" prop="tags">
          <el-select
            v-model="postForm.tags"
            multiple
            filterable
            allow-create
            placeholder="请选择标签"
          >
            <el-option
              v-for="tag in availableTags"
              :key="tag"
              :label="tag"
              :value="tag"
            />
          </el-select>
        </el-form-item>

        <el-form-item label="摘要" prop="summary">
          <el-input
            v-model="postForm.summary"
            type="textarea"
            :rows="3"
            placeholder="请输入文章摘要"
          />
        </el-form-item>

        <el-form-item label="内容" prop="content">
          <MdEditor
            v-model="postForm.content"
            :toolbars="editorConfig.toolbars"
            @onUploadImg="handleUploadImage"
            language="zh-CN"
            preview-theme="github"
            code-theme="github"
            style="height: 500px"
          />
        </el-form-item>

        <el-form-item label="状态" prop="status">
          <el-radio-group v-model="postForm.status">
            <el-radio label="draft">保存为草稿</el-radio>
            <el-radio label="published">直接发布</el-radio>
          </el-radio-group>
        </el-form-item>

        <el-form-item>
          <el-button type="primary" @click="handleSubmit(formRef)" :loading="loading">
            {{ isEdit ? '更新' : '发布' }}
          </el-button>
          <el-button @click="router.push('/admin/posts')">取消</el-button>
        </el-form-item>
      </el-form>
    </el-card>
  </div>
</template>

<style>
.md-editor {
  border: 1px solid var(--el-border-color);
  border-radius: 4px;
}
</style>