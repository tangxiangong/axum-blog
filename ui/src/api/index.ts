import axios from 'axios'
import type {
    AdminInfo,
    WebsiteInfo,
    SignInRequest,
    UpdateAdminRequest,
    UpdateWebsiteRequest
} from './types'

const api = axios.create({
    baseURL: import.meta.env.VITE_API_BASE_URL || 'http://localhost:3000',
    withCredentials: true
})

// 拦截器处理认证信息
api.interceptors.request.use(
    (config) => {
        const token = localStorage.getItem('token')
        if (token) {
            config.headers.Authorization = `Bearer ${token}`
        }
        return config
    },
    (error) => {
        return Promise.reject(error)
    }
)

// 管理员相关 API
export const adminApi = {
    signIn: (data: SignInRequest) =>
        api.post<AdminInfo>('/signin', data),

    signOut: () =>
        api.post('/signout'),

    getInfo: () =>
        api.get<AdminInfo>('/admin'),

    updateInfo: (data: UpdateAdminRequest) =>
        api.patch<AdminInfo>('/admin', data),

    updateAvatar: (file: File) => {
        const formData = new FormData()
        formData.append('file', file)
        return api.patch<AdminInfo>('/admin/upload', formData, {
            headers: {
                'Content-Type': 'multipart/form-data'
            }
        })
    }
}

// 网站相关 API
export const websiteApi = {
    getInfo: () =>
        api.get<WebsiteInfo>('/website'),

    updateInfo: (data: UpdateWebsiteRequest) =>
        api.patch<WebsiteInfo>('/website', data),

    updateLogo: (file: File) => {
        const formData = new FormData()
        formData.append('file', file)
        return api.patch<WebsiteInfo>('/website/upload/logo', formData, {
            headers: {
                'Content-Type': 'multipart/form-data'
            }
        })
    },

    updateFavicon: (file: File) => {
        const formData = new FormData()
        formData.append('file', file)
        return api.patch<WebsiteInfo>('/website/upload/favicon', formData, {
            headers: {
                'Content-Type': 'multipart/form-data'
            }
        })
    }
}

export default api 