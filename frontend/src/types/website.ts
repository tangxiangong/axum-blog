export interface Website {
  title: string;
  subtitle?: string;
  description?: string;
  logo?: string;
  favicon?: string;
  created_at: string;
  updated_at: string;
}

export interface UpdateWebsiteInfo {
  title?: string;
  subtitle?: string;
  description?: string;
}
