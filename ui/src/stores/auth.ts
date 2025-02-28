import { defineStore } from 'pinia'
import { ref } from 'vue'

interface UserInfo {
  id: number
  username: string
  email: string
  avatar?: string
}

export const useAuthStore = defineStore('auth', () => {
  const token = ref<string | null>(localStorage.getItem('token'))
  const userInfo = ref<UserInfo | null>(null)
  const isAuthenticated = ref(!!token.value)

  const login = async (username: string, password: string) => {
    try {
      // TODO: 实现实际的登录API调用
      const mockResponse = {
        token: 'mock_token_123',
        user: {
          id: 1,
          username: username,
          email: `${username}@example.com`,
          avatar: 'https://avatars.githubusercontent.com/u/1234567'
        }
      }

      token.value = mockResponse.token
      userInfo.value = mockResponse.user
      isAuthenticated.value = true
      
      localStorage.setItem('token', mockResponse.token)
      
      return true
    } catch (error) {
      console.error('Login error:', error)
      return false
    }
  }

  const logout = () => {
    token.value = null
    userInfo.value = null
    isAuthenticated.value = false
    localStorage.removeItem('token')
  }

  const updateUserInfo = (info: Partial<UserInfo>) => {
    if (userInfo.value) {
      userInfo.value = { ...userInfo.value, ...info }
    }
  }

  return {
    token,
    userInfo,
    isAuthenticated,
    login,
    logout,
    updateUserInfo
  }
}) 