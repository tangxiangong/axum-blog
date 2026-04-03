export interface Admin {
  id: string;
  name: string;
  password: number[];
  nickname?: string;
  email?: string;
  github?: string;
  wechat?: string;
  qq?: string;
  avatar?: string;
  created_at: string;
  updated_at: string;
}

export interface Login {
  username: string;
  password: string;
}

export interface RememberMe {
  remember_me: boolean;
}

export interface UpdateAdminInfo {
  name?: string;
  nickname?: string;
  email?: string;
  github?: string;
  wechat?: string;
  qq?: string;
}
