# axum-blog
Axum + Vue3 + MySQL 搭建的个人博客网站

## TODO
- [x] Authorization: JWT + Session
- [x] CORS + TIMEOUT
- [x] `website` 相关功能
- [x] `admin` 相关功能
- [ ] `category` 相关功能
  - [x] 添加
  - [x] 更新
  - [x] 删除
  - [x] 查询
    - [x] 根据父级ID查询
    - [x] 根据名称查询
  - [x] 列表
  - [x] handler 层
  - [ ] API 层
- [ ] `article` 相关功能
- [ ] `tag` 相关功能
  - [x] 添加
  - [x] 更新
  - [x] 删除
  - [x] 查询
  - [x] 列表
  - [ ] handler 层
  - [ ] API 层
- [ ] `comment` 相关功能
- [ ] 腾讯云 COS SDK
- [ ] 接入 DeepSeek
  - [x] [SDK](./crates/deepseek/)