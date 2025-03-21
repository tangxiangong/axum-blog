import request from '@/utils/request'
import type {
    Category,
    CreateCategoryRequest,
    UpdateCategoryRequest,
    PaginatedResponse
} from './types'

export const categoryApi = {
    /**
     * 获取分类列表
     */
    listCategories: () => {
        return request<PaginatedResponse<Category>>({
            url: '/api/categories',
            method: 'get'
        })
    },

    /**
     * 获取分类详情
     * @param id 分类ID
     */
    getCategory: (id: number) => {
        return request<Category>({
            url: `/api/categories/${id}`,
            method: 'get'
        })
    },

    /**
     * 创建分类
     * @param data 分类数据
     */
    createCategory: (data: CreateCategoryRequest) => {
        return request<Category>({
            url: '/api/categories',
            method: 'post',
            data
        })
    },

    /**
     * 更新分类
     * @param id 分类ID
     * @param data 分类数据
     */
    updateCategory: (id: number, data: UpdateCategoryRequest) => {
        return request<Category>({
            url: `/api/categories/${id}`,
            method: 'put',
            data
        })
    },

    /**
     * 删除分类
     * @param id 分类ID
     */
    deleteCategory: (id: number) => {
        return request<void>({
            url: `/api/categories/${id}`,
            method: 'delete'
        })
    }
} 