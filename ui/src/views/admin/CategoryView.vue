<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import type { FormInstance, FormRules } from 'element-plus'
import { categoryApi } from '@/api'
import type { Category } from '@/api/types'

const categories = ref<Category[]>([])
const loading = ref(false)
const dialogVisible = ref(false)
const formRef = ref<FormInstance>()
const editingCategory = ref<Category | null>(null)
const form = ref({
  name: '',
  parent_id: null as number | null
})

// 获取所有分类
const fetchCategories = async () => {
  try {
    loading.value = true
    const { data } = await categoryApi.getList()
    categories.value = data
  } catch (error) {
    console.error('获取分类列表失败:', error)
    ElMessage.error('获取分类列表失败')
  } finally {
    loading.value = false
  }
}

const rules = ref<FormRules>({
  name: [
    { required: true, message: '请输入分类名称', trigger: 'blur' },
    { min: 2, max: 20, message: '名称长度应在2-20个字符之间', trigger: 'blur' }
  ]
})

const handleEdit = (category: Category) => {
  editingCategory.value = category
  form.value = {
    name: category.name,
    parent_id: category.parent_id
  }
  dialogVisible.value = true
}

const handleAdd = () => {
  editingCategory.value = null
  form.value = {
    name: '',
    parent_id: null
  }
  dialogVisible.value = true
}

const handleDelete = async (id: number) => {
  try {
    await ElMessageBox.confirm('确定要删除这个分类吗？相关文章的分类将被清空', '提示', {
      type: 'warning'
    })
    
    loading.value = true
    await categoryApi.deleteById(id)
    ElMessage.success('删除成功')
    await fetchCategories() // 刷新列表
  } catch (error) {
    if (error instanceof Error) {
      console.error('删除分类失败:', error)
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
        
        if (editingCategory.value) {
          // 更新分类
          await categoryApi.update({
            id: editingCategory.value.id,
            name: form.value.name,
            parent_id: form.value.parent_id
          })
          ElMessage.success('更新成功')
        } else {
          // 创建分类
          await categoryApi.create({
            name: form.value.name,
            parent_id: form.value.parent_id
          })
          ElMessage.success('创建成功')
        }
        
        dialogVisible.value = false
        await fetchCategories() // 刷新列表
      } catch (error) {
        console.error(editingCategory.value ? '更新分类失败:' : '创建分类失败:', error)
        ElMessage.error(editingCategory.value ? '更新失败' : '创建失败')
      } finally {
        loading.value = false
      }
    }
  })
}

// 获取父分类的名称
const getParentCategoryName = (parentId: number | null) => {
  if (!parentId) return '-'
  const parent = categories.value.find(c => c.id === parentId)
  return parent?.name || '-'
}

onMounted(() => {
  fetchCategories()
})
</script>

<template>
  <div class="p-6">
    <div class="flex justify-between items-center mb-6">
      <h1 class="text-2xl font-bold">分类管理</h1>
      <el-button type="primary" @click="handleAdd">
        添加分类
      </el-button>
    </div>

    <el-card v-loading="loading">
      <el-table :data="categories" style="width: 100%">
        <el-table-column prop="id" label="ID" width="80" />
        <el-table-column prop="name" label="名称" width="180" />
        <el-table-column label="父分类" width="180">
          <template #default="{ row }">
            {{ getParentCategoryName(row.parent_id) }}
          </template>
        </el-table-column>
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
      :title="editingCategory ? '编辑分类' : '添加分类'"
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
          <el-input v-model="form.name" placeholder="请输入分类名称" />
        </el-form-item>
        <el-form-item label="父分类">
          <el-select 
            v-model="form.parent_id" 
            placeholder="请选择父分类" 
            clearable
            style="width: 100%"
          >
            <el-option 
              v-for="category in categories" 
              :key="category.id" 
              :label="category.name" 
              :value="category.id"
              :disabled="editingCategory && category.id === editingCategory.id"
            />
          </el-select>
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