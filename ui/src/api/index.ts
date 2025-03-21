import axios from 'axios'
import type {
    AdminInfo,
    WebsiteInfo,
    SignInRequest,
    UpdateAdminRequest,
    UpdateWebsiteRequest,
    Category,
    CreateCategoryRequest,
    UpdateCategoryRequest,
    Tag,
    CreateTagRequest,
    UpdateTagRequest,
    Article,
    ArticleQuery,
    CreateArticleRequest,
    UpdateArticleRequest
} from './types'
import { adminApi } from './adminApi'
import { articleApi } from './articleApi'
import { categoryApi } from './categoryApi'
import { tagApi } from './tagApi'

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

// 分类相关 API
export const categoryApi = {
    getList: () =>
        api.get<Category[]>('/category/list'),

    getById: (id: number) =>
        api.get<Category>(`/category/id?id=${id}`),

    getByName: (name: string) =>
        api.get<Category>(`/category/name?name=${name}`),

    getByParentId: (parentId: number) =>
        api.get<Category[]>(`/category/parent?parent_id=${parentId}`),

    create: (data: CreateCategoryRequest) =>
        api.post<void>('/category/add', data),

    update: (data: UpdateCategoryRequest) =>
        api.patch<void>('/category/update', data),

    deleteById: (id: number) =>
        api.delete<void>(`/category/id?id=${id}`),

    deleteByName: (name: string) =>
        api.delete<void>(`/category/name?name=${name}`)
}

// 标签相关 API
export const tagApi = {
    getList: () =>
        api.get<Tag[]>('/tag/list'),

    getById: (id: number) =>
        api.get<Tag>(`/tag/id?id=${id}`),

    getByName: (name: string) =>
        api.get<Tag>(`/tag/name?name=${name}`),

    create: (name: string) =>
        api.post<void>('/tag/add', { name }),

    update: (id: number, name: string) =>
        api.patch<void>('/tag/update', { id, name }),

    deleteById: (id: number) =>
        api.delete<void>(`/tag/id?id=${id}`),

    deleteByName: (name: string) =>
        api.delete<void>(`/tag/name?name=${name}`)
}

// 文章相关 API
export const articleApi = {
    getList: (params?: ArticleQuery) =>
        api.get<Article[]>('/article/list', { params }),

    getById: (id: number) =>
        api.get<Article>(`/article/id?id=${id}`),

    create: (data: CreateArticleRequest) =>
        api.post<void>('/article/add', data),

    update: (data: UpdateArticleRequest) =>
        api.patch<void>('/article/update', data),

    deleteById: (id: number) =>
        api.delete<void>(`/article/id?id=${id}`),

    publish: (id: number) =>
        api.patch<void>(`/article/publish?id=${id}`),

    unpublish: (id: number) =>
        api.patch<void>(`/article/unpublish?id=${id}`)
}

export default api

export {
    adminApi,
    articleApi,
    categoryApi,
    tagApi
} 