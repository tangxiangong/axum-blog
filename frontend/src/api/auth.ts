import type { Login } from "../types";
import { post } from "./client";

/**
 * POST /signin
 *
 * Content-Type: application/x-www-form-urlencoded
 * 查询参数: ?remember_me=true（可选）
 *
 * 响应头可能包含:
 *   - Set-Cookie: SESSION_ID=...
 *   - Bearer: <jwt_token>（当 remember_me=true 时）
 */
export async function signin(credentials: Login, rememberMe = false): Promise<{ token?: string }> {
  const query = rememberMe ? "?remember_me=true" : "";
  const res = await post(`/signin${query}`, {
    username: credentials.username,
    password: credentials.password,
  });
  const token = (res.headers["bearer"] as string) ?? undefined;
  return { token };
}

/**
 * POST /signout（需要认证）
 *
 * 删除 Redis 中的 Session，并将 JWT 加入黑名单。
 */
export async function signout(): Promise<void> {
  await post("/signout");
}
