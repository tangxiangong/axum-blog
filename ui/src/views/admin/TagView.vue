<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import type { FormInstance, FormRules } from 'element-plus'
import { tagApi } from '@/api'
import type { Tag } from '@/api/types'

const tags = ref<Tag[]>([])
const loading = ref(false)
const dialogVisible = ref(false)
const formRef = ref<FormInstance>()
const editingTag = ref<Tag | null>(null)
const form = ref({
  name: ''
})

// 获取所有标签
const fetchTags = async () => {
  try {
    loading.value = true
    const { data } = await tagApi.getList()
    tags.value = data
  } catch (error) {
    console.error('获取标签列表失败:', error)
    ElMessage.error('获取标签列表失败')
  } finally {
    loading.value = false
  }
}

const rules = ref<FormRules>({
  name: [
    { required: true, message: '请输入标签名称', trigger: 'blur' },
    { min: 1, max: 20, message: '名称长度应在1-20个字符之间', trigger: 'blur' }
  ]
})

const handleEdit = (tag: Tag) => {
  editingTag.value = tag
  form.value = {
    name: tag.name
  }
  dialogVisible.value = true
}

const handleAdd = () => {
  editingTag.value = null
  form.value = {
    name: ''
  }
  dialogVisible.value = true
}

const handleDelete = async (id: number) => {
  try {
    await ElMessageBox.confirm('确定要删除这个标签吗？相关文章的标签将被移除', '提示', {
      type: 'warning'
    })
    
    loading.value = true
    await tagApi.deleteById(id)
    ElMessage.success('删除成功')
    await fetchTags() // 刷新列表
  } catch (error) {
    if (error instanceof Error) {
      console.error('删除标签失败:', error)
      ElMessage.error('删除失败')
    }
    // 用户取消删除的情况不显示错误
  } finally {
    loading.value = false
  }
}

const handleSubmit = async (formEl: FormInstance | undefined) => {
  if (!formEl) return
  
  await formEl.validate(async (valid) => {
    if (valid) {
      try {
        loading.value = true
        
        if (editingTag.value) {
          // 更新标签
          await tagApi.update(editingTag.value.id, form.value.name)
          ElMessage.success('更新成功')
        } else {
          // 创建标签
          await tagApi.create(form.value.name)
          ElMessage.success('创建成功')
        }
        
        dialogVisible.value = false
        await fetchTags() // 刷新列表
      } catch (error) {
        console.error(editingTag.value ? '更新标签失败:' : '创建标签失败:', error)
        ElMessage.error(editingTag.value ? '更新失败' : '创建失败')
      } finally {
        loading.value = false
      }
    }
  })
}

onMounted(() => {
  fetchTags()
})
</script>

<template>
  <div class="p-6">
    <div class="flex justify-between items-center mb-6">
      <h1 class="text-2xl font-bold">标签管理</h1>
      <el-button type="primary" @click="handleAdd">
        添加标签
      </el-button>
    </div>

    <el-card v-loading="loading">
      <el-table :data="tags" style="width: 100%">
        <el-table-column prop="id" label="ID" width="80" />
        <el-table-column prop="name" label="名称" width="180" />
        <el-table-column prop="created_at" label="创建时间" width="180" />
        <el-table-column prop="updated_at" label="更新时间" width="180" />
        <el-table-column fixed="right" label="操作" width="150">
          <template #default="{ row }">
            <el-button
              link
              type="primary"
              @click="handleEdit(row)"
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

    <!-- 编辑/添加对话框 -->
    <el-dialog
      v-model="dialogVisible"
      :title="editingTag ? '编辑标签' : '添加标签'"
      width="500px"
      destroy-on-close
    >
      <el-form
        ref="formRef"
        :model="form"
        :rules="rules"
        label-width="100px"
        status-icon
      >
        <el-form-item label="名称" prop="name">
          <el-input v-model="form.name" placeholder="请输入标签名称" />
        </el-form-item>
      </el-form>
      <template #footer>
        <span class="dialog-footer">
          <el-button @click="dialogVisible = false">取消</el-button>
          <el-button type="primary" @click="handleSubmit(formRef)">
            确定
          </el-button>
        </span>
      </template>
    </el-dialog>
  </div>
</template> 