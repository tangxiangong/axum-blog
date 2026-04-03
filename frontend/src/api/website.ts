import type { AppResponse, Website, UpdateWebsiteInfo } from "../types";
import { get, patch, patchFile } from "./client";

/**
 * GET /website（公开）
 *
 * 获取网站公开信息。
 */
export async function getWebsite(): Promise<AppResponse<Website>> {
  return get<Website>("/website");
}

/**
 * PATCH /website（需要认证）
 *
 * Content-Type: application/x-www-form-urlencoded
 * 所有字段均为可选。
 */
export async function updateWebsite(info: UpdateWebsiteInfo): Promise<AppResponse> {
  return patch("/website", info as Record<string, string | undefined>);
}

/**
 * PATCH /website/upload/logo（需要认证）
 *
 * Content-Type: multipart/form-data
 * 上传单个图片文件作为网站 Logo。
 */
export async function updateLogo(file: File): Promise<AppResponse> {
  return patchFile("/website/upload/logo", file);
}

/**
 * PATCH /website/upload/favicon（需要认证）
 *
 * Content-Type: multipart/form-data
 * 上传单个图片文件作为网站 Favicon。
 */
export async function updateFavicon(file: File): Promise<AppResponse> {
  return patchFile("/website/upload/favicon", file);
}
