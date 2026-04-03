# 我的个人网站

## 技术栈

- 后端：Rust + Axum
- 数据库：SurrealDB
- 缓存：Redis
- 前端：SolidJS + TailwindCSS（`/frontend`）

## 本地启动

1. 启动依赖服务：

```bash
docker compose up -d
```

2. 启动后端：

```bash
cargo run
```

默认配置文件为 `configration.yml`，后端会在启动时自动执行 SurrealDB schema 初始化，并在空库时初始化管理员与网站基础信息。

3. 启动前端（可选）：

```bash
cd frontend
bun run dev
```
