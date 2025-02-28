<script setup lang="ts">
import { ref } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import type { FormInstance, FormRules } from 'element-plus'

interface Tag {
  id: number
  name: string
  postCount: number
  createdAt: string
}

const tags = ref<Tag[]>([
  {
    id: 1,
    name: 'Vue',
    postCount: 8,
    createdAt: '2024-03-10'
  },
  {
    id: 2,
    name: 'TypeScript',
    postCount: 5,
    createdAt: '2024-03-09'
  }
])

const dialogVisible = ref(false)
const formRef = ref<FormInstance>()
const editingTag = ref<Tag | null>(null)
const form = ref({
  name: ''
})

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
    // TODO: 实现实际的删除API调用
    tags.value = tags.value.filter(tag => tag.id !== id)
    ElMessage.success('删除成功')
  } catch {
    // 用户取消删除
  }
}

const handleSubmit = async (formEl: FormInstance | undefined) => {
  if (!formEl) return
  
  await formEl.validate(async (valid) => {
    if (valid) {
      try {
        // TODO: 实现实际的API调用
        if (editingTag.value) {
          // 更新
          const index = tags.value.findIndex(tag => tag.id === editingTag.value?.id)
          if (index !== -1) {
            tags.value[index] = {
              ...editingTag.value,
              ...form.value
            }
          }
          ElMessage.success('更新成功')
        } else {
          // 新增
          tags.value.push({
            id: Date.now(),
            name: form.value.name,
            postCount: 0,
            createdAt: new Date().toISOString().split('T')[0]
          })
          ElMessage.success('创建成功')
        }
        dialogVisible.value = false
      } catch (error) {
        ElMessage.error(editingTag.value ? '更新失败' : '创建失败')
      }
    }
  })
}
</script>

<template>
  <div class="p-6">
    <div class="flex justify-between items-center mb-6">
      <h1 class="text-2xl font-bold">标签管理</h1>
      <el-button type="primary" @click="handleAdd">
        添加标签
      </el-button>
    </div>

    <el-card>
      <el-table :data="tags" style="width: 100%">
        <el-table-column prop="name" label="名称" width="180" />
        <el-table-column prop="postCount" label="文章数" width="100" align="center" />
        <el-table-column prop="createdAt" label="创建时间" width="180" />
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
              :disabled="row.postCount > 0"
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