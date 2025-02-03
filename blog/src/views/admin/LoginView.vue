<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '@/stores/auth'

const router = useRouter()
const authStore = useAuthStore()

const loginForm = ref({
  username: '',
  password: ''
})

const loading = ref(false)
const errorMessage = ref('')

const handleLogin = async () => {
  loading.value = true
  errorMessage.value = ''
  
  try {
    const success = await authStore.login(loginForm.value.username, loginForm.value.password)
    if (success) {
      router.push('/admin')
    } else {
      errorMessage.value = '登录失败，请检查用户名和密码'
    }
  } catch (error) {
    errorMessage.value = '登录过程中发生错误'
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <div class="min-h-screen flex items-center justify-center bg-gray-50 py-12 px-4 sm:px-6 lg:px-8">
    <div class="max-w-md w-full space-y-8">
      <div>
        <h2 class="mt-6 text-center text-3xl font-extrabold text-gray-900">
          登录到管理后台
        </h2>
      </div>
      <form class="mt-8 space-y-6" @submit.prevent="handleLogin">
        <div class="rounded-md shadow-sm -space-y-px">
          <div>
            <el-input
              v-model="loginForm.username"
              type="text"
              required
              placeholder="用户名"
              class="rounded-t-md"
            />
          </div>
          <div>
            <el-input
              v-model="loginForm.password"
              type="password"
              required
              placeholder="密码"
              class="rounded-b-md"
            />
          </div>
        </div>

        <div>
          <el-button
            type="primary"
            :loading="loading"
            class="w-full"
            @click="handleLogin"
          >
            登录
          </el-button>
        </div>
        
        <el-alert
          v-if="errorMessage"
          :title="errorMessage"
          type="error"
          show-icon
        />
      </form>
    </div>
  </div>
</template> 