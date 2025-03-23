<script setup lang="ts">
import { ref } from 'vue'
import { ElMessage } from 'element-plus'
import { adminApi } from '@/api'
import type { AdminInfo } from '@/api/types'

const adminInfo = ref<AdminInfo>()
const loading = ref(false)

// 获取管理员信息
const getAdminInfo = async () => {
  try {
    loading.value = true
    const { data } = await adminApi.getInfo()
    adminInfo.value = data
  } catch (error: any) {
    console.error('获取管理员信息失败:', error)
    const errorMessage = error.response?.data?.message || '获取管理员信息失败'
    ElMessage.error(errorMessage)
  } finally {
    loading.value = false
  }
}

// 更新管理员信息
const handleUpdate = async () => {
  if (!adminInfo.value) return
  
  try {
    loading.value = true
    await adminApi.updateInfo({
      nickname: adminInfo.value.nickname
    })
    ElMessage.success('更新成功')
  } catch (error: any) {
    console.error('更新管理员信息失败:', error)
    const errorMessage = error.response?.data?.message || '更新管理员信息失败'
    ElMessage.error(errorMessage)
  } finally {
    loading.value = false
  }
}

// 更新头像
const handleAvatarUpload = async (file: File) => {
  try {
    loading.value = true
    await adminApi.updateAvatar(file)
    await getAdminInfo() // 刷新数据
    ElMessage.success('头像更新成功')
  } catch (error: any) {
    console.error('上传头像失败:', error)
    const errorMessage = error.response?.data?.message || '上传头像失败'
    ElMessage.error(errorMessage)
  } finally {
    loading.value = false
  }
}

// 组件挂载时获取数据
getAdminInfo()
</script>

<template>
  <div class="p-6">
    <el-card v-loading="loading">
      <template #header>
        <div class="flex items-center justify-between">
          <span>个人资料</span>
          <el-button type="primary" @click="handleUpdate">保存更改</el-button>
        </div>
      </template>

      <el-form label-width="100px" v-if="adminInfo">
        <el-form-item label="用户名">
          <el-input v-model="adminInfo.username" disabled />
        </el-form-item>

        <el-form-item label="昵称">
          <el-input v-model="adminInfo.nickname" />
        </el-form-item>

        <el-form-item label="头像">
          <div class="flex items-center space-x-4">
            <el-avatar
              :size="64"
              :src="adminInfo.avatar"
            />
            <el-upload
              :http-request="({ file }: { file: File }) => handleAvatarUpload(file)"
              :show-file-list="false"
              accept="image/*"
            >
              <el-button type="primary">更换头像</el-button>
            </el-upload>
          </div>
        </el-form-item>

        <el-form-item label="注册时间">
          <el-input :model-value="adminInfo.created_at" disabled />
        </el-form-item>

        <el-form-item label="更新时间">
          <el-input :model-value="adminInfo.updated_at" disabled />
        </el-form-item>
      </el-form>
    </el-card>
  </div>
</template> 