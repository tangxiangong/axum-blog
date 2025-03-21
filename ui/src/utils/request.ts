import axios, { AxiosRequestConfig, AxiosResponse } from 'axios'
import { ElMessage } from 'element-plus'
import router from '@/router'

// 创建axios实例
const service = axios.create({
    baseURL: import.meta.env.VITE_API_BASE_URL || '/api',
    timeout: 15000
})

// 请求拦截器
service.interceptors.request.use(
    config => {
        // 添加token到请求头
        const token = localStorage.getItem('token')
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

            switch (status) {
                case 401:
                    // 未授权，清除token并跳转到登录页
                    localStorage.removeItem('token')
                    router.push({
                        path: '/login',
                        query: { redirect: router.currentRoute.value.fullPath }
                    })
                    ElMessage.error('登录已过期，请重新登录')
                    break
                case 403:
                    ElMessage.error('没有权限访问此资源')
                    break
                case 404:
                    ElMessage.error('请求的资源不存在')
                    break
                case 500:
                    ElMessage.error('服务器错误，请稍后再试')
                    break
                default:
                    // 显示后端返回的错误信息或默认信息
                    ElMessage.error(data?.message || '请求失败，请稍后再试')
            }
        } else if (error.request) {
            // 请求发出但没有收到响应
            ElMessage.error('网络错误，请检查您的网络连接')
        } else {
            // 设置请求时发生错误
            ElMessage.error('请求错误：' + error.message)
        }

        return Promise.reject(error)
    }
)

// 请求函数封装
const request = <T = any>(config: AxiosRequestConfig): Promise<AxiosResponse<T>> => {
    return service(config)
}

export default request 