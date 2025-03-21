import request from '@/utils/request'
import type {
    AdminInfo,
    SignInRequest,
    UpdateAdminRequest,
    WebsiteInfo,
    UpdateWebsiteRequest
} from './types'

export const adminApi = {
    /**
     * 管理员登录
     * @param data 登录参数
     */
    signIn: (data: SignInRequest) => {
        return request({
            url: '/api/admin/signin',
            method: 'post',
            data
        })
    },

    /**
     * 管理员登出
     */
    signOut: () => {
        return request<void>({
            url: '/api/admin/signout',
            method: 'post'
        })
    },

    /**
     * 获取管理员信息
     */
    getAdminInfo: () => {
        return request<AdminInfo>({
            url: '/api/admin/info',
            method: 'get'
        })
    },

    /**
     * 更新管理员信息
     * @param data 更新参数
     */
    updateAdminInfo: (data: UpdateAdminRequest) => {
        return request<AdminInfo>({
            url: '/api/admin/info',
            method: 'put',
            data
        })
    },

    /**
     * 获取网站信息
     */
    getWebsiteInfo: () => {
        return request<WebsiteInfo>({
            url: '/api/admin/website',
            method: 'get'
        })
    },

    /**
     * 更新网站信息
     * @param data 更新参数
     */
    updateWebsiteInfo: (data: UpdateWebsiteRequest) => {
        return request<WebsiteInfo>({
            url: '/api/admin/website',
            method: 'put',
            data
        })
    }
} 