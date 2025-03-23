import axios, { AxiosRequestConfig, AxiosResponse } from 'axios'
import { ElMessage } from 'element-plus'
import router from '@/router'
import { JWT_TOKEN_KEY } from '@/api'

// 创建axios实例
const service = axios.create({
    baseURL: import.meta.env.VITE_API_BASE_URL || '/api',
    timeout: 15000,
    withCredentials: true // 启用跨域请求时携带cookie
})

// 请求拦截器
service.interceptors.request.use(
    config => {
        // 从localStorage获取JWT令牌并添加到请求头
        const token = localStorage.getItem(JWT_TOKEN_KEY)
        if (token) {
            config.headers['Authorization'] = `Bearer ${token}`
        }
        return config
    },
    error => {
        console.error('请求错误:', error)
        return Promise.reject(error)
    }
)

// 响应拦截器
service.interceptors.response.use(
    response => {
        return response
    },
    error => {
        // 处理错误响应
        if (error.response) {
            const { status, data } = error.response

            // 提取错误信息
            const errorMessage = data?.message || '未知错误'

            switch (status) {
                case 400:
                    // 400 表示客户端请求有误
                    ElMessage.error(`请求参数错误: ${errorMessage}`)
                    break
                case 401:
                    // 未授权，跳转到登录页
                    router.push({
                        path: '/login',
                        query: { redirect: router.currentRoute.value.fullPath }
                    })
                    ElMessage.error(`登录已过期，请重新登录: ${errorMessage}`)
                    break
                case 403:
                    ElMessage.error(`没有权限访问此资源: ${errorMessage}`)
                    break
                case 404:
                    ElMessage.error(`请求的资源不存在: ${errorMessage}`)
                    break
                case 409:
                    // 冲突错误
                    ElMessage.error(`操作冲突: ${errorMessage}`)
                    break
                case 422:
                    // 表单验证错误
                    ElMessage.error(`表单验证失败: ${errorMessage}`)
                    break
                case 500:
                    ElMessage.error(`服务器错误: ${errorMessage}`)
                    break
                default:
                    // 显示后端返回的错误信息或默认信息
                    ElMessage.error(errorMessage)
            }
        } else if (error.request) {
            // 请求发出但没有收到响应
            ElMessage.error('网络错误，请检查您的网络连接')
        } else {
            // 设置请求时发生错误
            ElMessage.error('请求错误：' + error.message)
        }

        // 在控制台输出详细错误信息，帮助开发调试
        console.error('API请求错误:', error.response?.data || error.message)

        return Promise.reject(error)
    }
)

// 请求函数封装
const request = <T = any>(config: AxiosRequestConfig): Promise<AxiosResponse<T>> => {
    return service(config)
}

export default request 