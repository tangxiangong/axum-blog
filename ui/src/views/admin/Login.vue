<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { ElMessage } from 'element-plus'
import { User, Lock } from '@element-plus/icons-vue'
import { adminApi } from '@/api'
import type { SignInRequest } from '@/api/types'

const router = useRouter()

const form = ref<SignInRequest>({
  username: '',
  password: ''
})

const loading = ref(false)
const rememberMe = ref(false)

const handleLogin = async () => {
  if (!form.value.username || !form.value.password) {
    ElMessage.warning('请输入用户名和密码')
    return
  }
  
  try {
    loading.value = true
    const loginParams: SignInRequest = {
      username: form.value.username,
      password: form.value.password,
      remember_me: rememberMe.value
    }
    await adminApi.signIn(loginParams)
    ElMessage.success('登录成功')
    
    // 存储用户名（如果勾选了记住我）
    if (rememberMe.value) {
      localStorage.setItem('username', form.value.username)
    } else {
      localStorage.removeItem('username')
    }
    
    router.push('/admin')
  } catch (error: any) {
    console.error('登录错误:', error)
    
    // 显示更具体的错误信息
    const errorMessage = error.response?.data?.message
    if (errorMessage) {
      if (errorMessage.includes('密码错误')) {
        ElMessage.error('密码错误，请重新输入')
      } else if (errorMessage.includes('用户名')) {
        ElMessage.error('用户名不存在或格式错误')
      } else {
        ElMessage.error(errorMessage)
      }
    } else {
      ElMessage.error('登录失败，请稍后重试')
    }
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <div class="login-page">
    <div class="login-container">
      <div class="login-form-container">
        <div class="login-header">
          <div class="logo">
            <i class="fas fa-feather-alt"></i>
          </div>
          <h1 class="title">小雨博客</h1>
          <p class="subtitle">管理员登录</p>
        </div>
        
        <el-form @submit.prevent="handleLogin" class="login-form">
          <el-form-item>
            <el-input
              v-model="form.username"
              placeholder="用户名"
              :prefix-icon="User"
              class="custom-input"
            />
          </el-form-item>
          
          <el-form-item>
            <el-input
              v-model="form.password"
              type="password"
              placeholder="密码"
              :prefix-icon="Lock"
              show-password
              class="custom-input"
              @keyup.enter="handleLogin"
            />
          </el-form-item>
          
          <div class="form-options">
            <el-checkbox v-model="rememberMe" label="记住我" size="small" />
            <a href="#" class="forgot-link">忘记密码?</a>
          </div>
          
          <el-form-item>
            <el-button
              type="primary"
              :loading="loading"
              class="login-button"
              @click="handleLogin"
            >
              {{ loading ? '登录中...' : '登 录' }}
            </el-button>
          </el-form-item>
        </el-form>
        
        <div class="login-footer">
          <p>© {{ new Date().getFullYear() }} 小雨博客. 保留所有权利.</p>
          <a href="/" class="back-to-site">
            <i class="fas fa-arrow-left"></i> 返回网站
          </a>
        </div>
      </div>
      
      <div class="login-decoration">
        <div class="decoration-content">
          <h2>欢迎回来!</h2>
          <p>登录后台管理系统，开始创作精彩内容</p>
          <div class="decoration-image">
            <img src="https://cdn.pixabay.com/photo/2018/09/04/10/27/laptop-3653422_960_720.jpg" alt="博客管理" />
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.login-page {
  min-height: 100vh;
  width: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  background-color: #f9fafb;
  padding: 1rem;
}

.login-container {
  width: 100%;
  max-width: 1000px;
  min-height: 600px;
  display: flex;
  border-radius: 16px;
  overflow: hidden;
  box-shadow: 0 10px 25px rgba(0, 0, 0, 0.08);
  background-color: white;
}

.login-form-container {
  flex: 1;
  padding: 2.5rem;
  display: flex;
  flex-direction: column;
}

.login-header {
  text-align: center;
  margin-bottom: 2rem;
}

.logo {
  width: 64px;
  height: 64px;
  background-color: #3b82f6;
  border-radius: 16px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  font-size: 1.75rem;
  margin: 0 auto 1rem;
  box-shadow: 0 10px 15px -3px rgba(59, 130, 246, 0.3);
  animation: pulse 2s infinite;
}

@keyframes pulse {
  0% {
    box-shadow: 0 0 0 0 rgba(59, 130, 246, 0.6);
  }
  70% {
    box-shadow: 0 0 0 10px rgba(59, 130, 246, 0);
  }
  100% {
    box-shadow: 0 0 0 0 rgba(59, 130, 246, 0);
  }
}

.title {
  font-size: 1.75rem;
  font-weight: 700;
  color: #1f2937;
  margin: 0;
}

.subtitle {
  font-size: 1rem;
  color: #6b7280;
  margin: 0.5rem 0 0;
}

.login-form {
  margin-top: 1rem;
  width: 100%;
  flex: 1;
}

.custom-input :deep(.el-input__wrapper) {
  padding: 0.75rem 1rem;
  border-radius: 10px;
  box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
  border: 1px solid #e5e7eb;
  transition: all 0.3s ease;
}

.custom-input:hover :deep(.el-input__wrapper) {
  border-color: #3b82f6;
}

.form-options {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 1.5rem;
}

.forgot-link {
  color: #3b82f6;
  text-decoration: none;
  font-size: 0.875rem;
  transition: color 0.2s ease;
}

.forgot-link:hover {
  color: #2563eb;
  text-decoration: underline;
}

.login-button {
  width: 100%;
  padding: 0.75rem;
  font-size: 1rem;
  font-weight: 500;
  border-radius: 10px;
  background-color: #3b82f6;
  transition: all 0.3s ease;
  border: none;
}

.login-button:hover {
  background-color: #2563eb;
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(37, 99, 235, 0.2);
}

.login-footer {
  margin-top: 2rem;
  text-align: center;
  font-size: 0.875rem;
  color: #6b7280;
}

.back-to-site {
  display: inline-block;
  margin-top: 0.5rem;
  color: #3b82f6;
  text-decoration: none;
  transition: all 0.2s ease;
}

.back-to-site:hover {
  color: #2563eb;
  transform: translateX(-3px);
}

.login-decoration {
  flex: 1;
  background: linear-gradient(135deg, #3b82f6 0%, #2563eb 100%);
  color: white;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 2rem;
  clip-path: polygon(8% 0, 100% 0, 100% 100%, 0 100%);
}

.decoration-content {
  max-width: 400px;
  text-align: center;
}

.decoration-content h2 {
  font-size: 2rem;
  font-weight: 700;
  margin: 0 0 1rem;
}

.decoration-content p {
  font-size: 1.1rem;
  opacity: 0.9;
  margin-bottom: 2rem;
}

.decoration-image {
  border-radius: 16px;
  overflow: hidden;
  box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04);
  transform: perspective(1000px) rotateY(-10deg);
  transition: transform 0.5s ease;
}

.decoration-image:hover {
  transform: perspective(1000px) rotateY(0deg);
}

.decoration-image img {
  width: 100%;
  height: auto;
  object-fit: cover;
  display: block;
}

@media (max-width: 768px) {
  .login-container {
    flex-direction: column;
  }
  
  .login-decoration {
    display: none;
  }
  
  .login-form-container {
    padding: 2rem 1.5rem;
  }
}
</style> 