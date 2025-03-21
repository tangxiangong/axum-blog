<script setup lang="ts">
import { ref, reactive } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { ElMessage } from 'element-plus'
import { adminApi } from '@/api'
import type { SignInRequest } from '@/api/types'

const router = useRouter()
const route = useRoute()
const loading = ref(false)

// 记住我
const rememberMe = ref(false)

// 登录表单
const loginForm = reactive<SignInRequest>({
  username: '',
  password: ''
})

// 表单规则
const rules = {
  username: [
    { required: true, message: '请输入用户名', trigger: 'blur' },
    { min: 3, max: 20, message: '用户名长度应为3-20个字符', trigger: 'blur' }
  ],
  password: [
    { required: true, message: '请输入密码', trigger: 'blur' },
    { min: 6, max: 30, message: '密码长度应为6-30个字符', trigger: 'blur' }
  ]
}

// 表单引用
const formRef = ref()

// 提交表单
const handleSubmit = async () => {
  if (!formRef.value) return

  await formRef.value.validate(async (valid: boolean) => {
    if (valid) {
      try {
        loading.value = true
        
        // 添加remember_me参数
        const params: SignInRequest = {
          ...loginForm,
          remember_me: rememberMe.value
        }
        
        const response = await adminApi.signIn(params)
        
        // 检查是否有Bearer头部（当remember_me为true时服务器会返回）
        const bearerToken = response.headers['bearer']
        if (bearerToken) {
          localStorage.setItem('token', bearerToken)
        }
        
        // 登录成功后跳转
        const redirectPath = route.query.redirect as string || '/admin'
        router.push(redirectPath)
        
        ElMessage.success('登录成功')
      } catch (error) {
        console.error('登录失败:', error)
        ElMessage.error('登录失败，请检查用户名和密码')
      } finally {
        loading.value = false
      }
    }
  })
}
</script>

<template>
  <div class="min-h-screen flex items-center justify-center bg-gray-100 dark:bg-gray-900">
    <div class="max-w-md w-full p-8 bg-white dark:bg-gray-800 rounded-lg shadow-lg">
      <div class="text-center mb-8">
        <h1 class="text-3xl font-bold text-gray-900 dark:text-white">博客管理系统</h1>
        <p class="text-gray-600 dark:text-gray-400 mt-2">请登录以继续</p>
      </div>
      
      <el-form
        ref="formRef"
        :model="loginForm"
        :rules="rules"
        label-position="top"
        @keyup.enter="handleSubmit"
      >
        <el-form-item label="用户名" prop="username">
          <el-input 
            v-model="loginForm.username" 
            placeholder="请输入用户名"
            prefix-icon="User"
          />
        </el-form-item>
        
        <el-form-item label="密码" prop="password">
          <el-input 
            v-model="loginForm.password" 
            type="password" 
            placeholder="请输入密码"
            prefix-icon="Lock" 
            show-password
          />
        </el-form-item>
        
        <div class="flex items-center justify-between mb-6">
          <el-checkbox v-model="rememberMe">记住我</el-checkbox>
          <a href="#" class="text-sm text-blue-500 hover:text-blue-700">忘记密码?</a>
        </div>
        
        <el-button 
          type="primary" 
          class="w-full" 
          :loading="loading" 
          @click="handleSubmit"
        >
          登录
        </el-button>
      </el-form>
    </div>
  </div>
</template>

<style scoped>
.el-form-item {
  margin-bottom: 20px;
}
</style> 