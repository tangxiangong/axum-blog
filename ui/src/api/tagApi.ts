import request from '@/utils/request'
import type {
    Tag,
    CreateTagRequest,
    UpdateTagRequest,
    PaginatedResponse
} from './types'

export const tagApi = {
    /**
     * 获取标签列表
     */
    listTags: () => {
        return request<PaginatedResponse<Tag>>({
            url: '/api/tags',
            method: 'get'
        })
    },

    /**
     * 获取标签详情
     * @param id 标签ID
     */
    getTag: (id: number) => {
        return request<Tag>({
            url: `/api/tags/${id}`,
            method: 'get'
        })
    },

    /**
     * 创建标签
     * @param data 标签数据
     */
    createTag: (data: CreateTagRequest) => {
        return request<Tag>({
            url: '/api/tags',
            method: 'post',
            data
        })
    },

    /**
     * 更新标签
     * @param id 标签ID
     * @param data 标签数据
     */
    updateTag: (id: number, data: UpdateTagRequest) => {
        return request<Tag>({
            url: `/api/tags/${id}`,
            method: 'put',
            data
        })
    },

    /**
     * 删除标签
     * @param id 标签ID
     */
    deleteTag: (id: number) => {
        return request<void>({
            url: `/api/tags/${id}`,
            method: 'delete'
        })
    }
} 