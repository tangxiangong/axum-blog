import type { AppResponse, Admin, UpdateAdminInfo } from "../types";
import { get, patch, patchFile } from "./client";

/**
 * GET /admin（需要认证）
 *
 * 获取当前已认证管理员的信息。
 */
export async function getAdmin(): Promise<AppResponse<Admin>> {
  return get<Admin>("/admin");
}

/**
 * PATCH /admin（需要认证）
 *
 * Content-Type: application/x-www-form-urlencoded
 * 所有字段均为可选。
 */
export async function updateAdmin(info: UpdateAdminInfo): Promise<AppResponse> {
  return patch("/admin", info as Record<string, string | undefined>);
}

/**
 * PATCH /admin/upload（需要认证）
 *
 * Content-Type: multipart/form-data
 * 上传单个图片文件作为管理员头像。
 */
export async function updateAvatar(file: File): Promise<AppResponse> {
  return patchFile("/admin/upload", file);
}
