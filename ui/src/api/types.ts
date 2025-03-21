export interface AdminInfo {
    id: number;
    username: string;
    nickname: string;
    avatar: string;
    created_at: string;
    updated_at: string;
}

export interface WebsiteInfo {
    title: string;
    subtitle: string;
    description: string;
    keywords: string[];
    logo: string;
    favicon: string;
    created_at: string;
    updated_at: string;
}

export interface SignInRequest {
    username: string;
    password: string;
    remember_me?: boolean;
}

export interface UpdateAdminRequest {
    nickname?: string;
    password?: string;
}

export interface UpdateWebsiteRequest {
    title?: string;
    subtitle?: string;
    description?: string;
    keywords?: string[];
}

export interface Category {
    id: number;
    name: string;
    parent_id: number | null;
    created_at: string;
    updated_at: string;
}

export interface CreateCategoryRequest {
    name: string;
    parent_id?: number;
}

export interface UpdateCategoryRequest {
    id: number;
    name: string;
    parent_id?: number;
}

export interface Tag {
    id: number;
    name: string;
    created_at: string;
    updated_at: string;
}

export interface CreateTagRequest {
    name: string;
}

export interface UpdateTagRequest {
    id: number;
    name: string;
}

export interface Article {
    id: number;
    title: string;
    summary: string | null;
    content: string;
    views: number;
    published: boolean;
    category_id: number | undefined;
    tag_ids: number[] | undefined;
    created_at: string;
    updated_at: string;
}

export interface ArticleQuery {
    page?: number;
    page_size?: number;
    category_id?: number;
    tag_id?: number;
    keyword?: string;
}

export interface CreateArticleRequest {
    title: string;
    summary?: string;
    content: string;
    published: boolean;
    category_id?: number;
    tag_ids?: number[];
}

export interface UpdateArticleRequest {
    title?: string;
    summary?: string;
    content?: string;
    published?: boolean;
    category_id?: number;
    tag_ids?: number[];
}

// 分页响应
export interface PaginatedResponse<T> {
    items: T[];
    total: number;
    page: number;
    page_size: number;
}

// 文章列表项
export interface ArticleListItem {
    id: number;
    title: string;
    summary: string | null;
    category_id: number | undefined;
    tag_ids: number[] | undefined;
    views: number;
    published: boolean;
    created_at: string;
    updated_at: string;
}

// 文章查询参数
export interface ListArticlesRequest {
    page?: number;
    page_size?: number;
    title?: string;
    category_id?: number;
    tag_id?: number;
    published?: boolean;
} 