import request from '@/utils/request'
import type {
    Article,
    ArticleListItem,
    CreateArticleRequest,
    UpdateArticleRequest,
    ListArticlesRequest,
    PaginatedResponse
} from './types'

export const articleApi = {
    /**
     * 获取文章列表
     * @param params 查询参数
     */
    listArticles: (params: ListArticlesRequest) => {
        return request<PaginatedResponse<ArticleListItem>>({
            url: '/api/articles',
            method: 'get',
            params
        })
    },

    /**
     * 获取文章详情
     * @param id 文章ID
     */
    getArticle: (id: number) => {
        return request<Article>({
            url: `/api/articles/${id}`,
            method: 'get'
        })
    },

    /**
     * 创建文章
     * @param data 文章数据
     */
    createArticle: (data: CreateArticleRequest) => {
        return request<Article>({
            url: '/api/articles',
            method: 'post',
            data
        })
    },

    /**
     * 更新文章
     * @param id 文章ID
     * @param data 文章数据
     */
    updateArticle: (id: number, data: UpdateArticleRequest) => {
        return request<Article>({
            url: `/api/articles/${id}`,
            method: 'put',
            data
        })
    },

    /**
     * 删除文章
     * @param id 文章ID
     */
    deleteArticle: (id: number) => {
        return request<void>({
            url: `/api/articles/${id}`,
            method: 'delete'
        })
    }
} 