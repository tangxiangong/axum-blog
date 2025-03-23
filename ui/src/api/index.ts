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

const api = axios.create({
    baseURL: import.meta.env.VITE_API_BASE_URL || '',
    withCredentials: true
})

// JWT 令牌的本地存储键名
export const JWT_TOKEN_KEY = 'blog_admin_token'

// 拦截器处理认证信息
api.interceptors.request.use(
    (config) => {
        // 如果用户选择了"记住我"，尝试从localStorage获取JWT
        const token = localStorage.getItem(JWT_TOKEN_KEY)
        if (token) {
            // 添加JWT到请求头的Authorization字段
            config.headers['Authorization'] = `Bearer ${token}`
        }
        // Cookie认证会自动处理（withCredentials: true）
        return config
    },
    (error) => {
        return Promise.reject(error)
    }
)

// 响应拦截器处理JWT令牌
api.interceptors.response.use(
    (response) => {
        // 检查响应头中是否包含JWT令牌
        const token = response.headers['authorization']
        if (token && token.startsWith('Bearer ')) {
            // 存储JWT令牌到localStorage
            localStorage.setItem(JWT_TOKEN_KEY, token.substring(7))
        }
        return response
    },
    (error) => {
        // 如果是401错误，清除可能过期的token
        if (error.response && error.response.status === 401) {
            localStorage.removeItem(JWT_TOKEN_KEY)
        }
        return Promise.reject(error)
    }
)

// 管理员相关 API
export const adminApi = {
    signIn: (data: SignInRequest) => {
        // 构建查询参数，确保 remember_me 作为 URL 参数传递
        const params = new URLSearchParams(data as any)
        const queryString = data.remember_me ? `?remember_me=true` : ''

        return api.post<AdminInfo>(`/signin${queryString}`, params, {
            headers: {
                'Content-Type': 'application/x-www-form-urlencoded'
            },
            transformResponse: [(data, headers) => {
                // 从响应头中获取JWT令牌并保存
                const bearerToken = headers?.bearer || headers?.Bearer
                if (bearerToken) {
                    localStorage.setItem(JWT_TOKEN_KEY, bearerToken)
                }
                return JSON.parse(data)
            }]
        })
    },

    signOut: () => {
        // 清除本地存储的JWT令牌
        localStorage.removeItem(JWT_TOKEN_KEY)
        return api.post('/signout', null, {
            headers: {
                'Content-Type': 'application/x-www-form-urlencoded'
            }
        })
    },

    getInfo: () =>
        api.get<AdminInfo>('/admin'),

    updateInfo: (data: UpdateAdminRequest) =>
        api.patch<AdminInfo>('/admin', new URLSearchParams(data as any), {
            headers: {
                'Content-Type': 'application/x-www-form-urlencoded'
            }
        }),

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