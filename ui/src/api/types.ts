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