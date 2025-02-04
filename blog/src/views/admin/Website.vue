<script setup lang="ts">
import { ref } from 'vue'
import { ElMessage } from 'element-plus'
import { websiteApi } from '@/api'
import type { WebsiteInfo } from '@/api/types'

const websiteInfo = ref<WebsiteInfo>()
const loading = ref(false)

// 获取网站信息
const getWebsiteInfo = async () => {
  try {
    loading.value = true
    const { data } = await websiteApi.getInfo()
    websiteInfo.value = data
  } catch (error) {
    console.error('获取网站信息失败:', error)
    ElMessage.error('获取网站信息失败')
  } finally {
    loading.value = false
  }
}

// 更新网站信息
const handleUpdate = async () => {
  if (!websiteInfo.value) return
  
  try {
    loading.value = true
    const { title, subtitle, description, keywords } = websiteInfo.value
    await websiteApi.updateInfo({
      title,
      subtitle,
      description,
      keywords
    })
    ElMessage.success('更新成功')
  } catch (error) {
    console.error('更新网站信息失败:', error)
    ElMessage.error('更新失败')
  } finally {
    loading.value = false
  }
}

// 上传Logo
const handleLogoUpload = async (file: File) => {
  try {
    loading.value = true
    await websiteApi.updateLogo(file)
    await getWebsiteInfo() // 刷新数据
    ElMessage.success('Logo更新成功')
  } catch (error) {
    console.error('上传Logo失败:', error)
    ElMessage.error('上传失败')
  } finally {
    loading.value = false
  }
}

// 上传Favicon
const handleFaviconUpload = async (file: File) => {
  try {
    loading.value = true
    await websiteApi.updateFavicon(file)
    await getWebsiteInfo() // 刷新数据
    ElMessage.success('Favicon更新成功')
  } catch (error) {
    console.error('上传Favicon失败:', error)
    ElMessage.error('上传失败')
  } finally {
    loading.value = false
  }
}

// 组件挂载时获取数据
getWebsiteInfo()
</script>

<template>
  <div class="p-6">
    <el-card v-loading="loading">
      <template #header>
        <div class="flex items-center justify-between">
          <span>网站设置</span>
          <el-button type="primary" @click="handleUpdate">保存更改</el-button>
        </div>
      </template>

      <el-form label-width="100px" v-if="websiteInfo">
        <el-form-item label="网站标题">
          <el-input v-model="websiteInfo.title" />
        </el-form-item>

        <el-form-item label="副标题">
          <el-input v-model="websiteInfo.subtitle" />
        </el-form-item>

        <el-form-item label="网站描述">
          <el-input
            v-model="websiteInfo.description"
            type="textarea"
            :rows="3"
          />
        </el-form-item>

        <el-form-item label="关键词">
          <el-select
            v-model="websiteInfo.keywords"
            multiple
            filterable
            allow-create
            default-first-option
            class="w-full"
            placeholder="请输入关键词"
          />
        </el-form-item>

        <el-form-item label="Logo">
          <div class="flex items-center space-x-4">
            <el-image
              :src="websiteInfo.logo"
              class="w-32 h-32 object-cover"
            />
            <el-upload
              :http-request="({ file }: { file: File }) => handleLogoUpload(file)"
              :show-file-list="false"
              accept="image/*"
            >
              <el-button type="primary">更换Logo</el-button>
            </el-upload>
          </div>
        </el-form-item>

        <el-form-item label="Favicon">
          <div class="flex items-center space-x-4">
            <el-image
              :src="websiteInfo.favicon"
              class="w-8 h-8 object-cover"
            />
            <el-upload
              :http-request="({ file }: { file: File }) => handleFaviconUpload(file)"
              :show-file-list="false"
              accept="image/*"
            >
              <el-button type="primary">更换Favicon</el-button>
            </el-upload>
          </div>
        </el-form-item>
      </el-form>
    </el-card>
  </div>
</template> 