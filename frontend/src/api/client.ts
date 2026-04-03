import axios, { AxiosError } from "axios";
import type { AppResponse, AppError } from "../types";

export const api = axios.create({
  withCredentials: true,
});

api.interceptors.response.use(
  res => res,
  (error: AxiosError<AppError>) => {
    if (error.response?.data) {
      const { code, status, message } = error.response.data;
      throw new ApiError({ code, status, message });
    }
    throw error;
  },
);

export class ApiError extends Error {
  code: number;
  status: string;

  constructor(error: AppError) {
    super(error.message);
    this.code = error.code;
    this.status = error.status;
  }
}

function toFormUrlEncoded(data: Record<string, string | undefined>): URLSearchParams {
  const params = new URLSearchParams();
  for (const [key, value] of Object.entries(data)) {
    if (value !== undefined) {
      params.set(key, value);
    }
  }
  return params;
}

export async function get<T = undefined, M = undefined>(path: string): Promise<AppResponse<T, M>> {
  const res = await api.get<AppResponse<T, M>>(path);
  return res.data;
}

export async function post(path: string, body?: Record<string, string | undefined>) {
  return api.post(path, body ? toFormUrlEncoded(body) : undefined, {
    headers: body ? { "Content-Type": "application/x-www-form-urlencoded" } : undefined,
  });
}

export async function patch<T = undefined, M = undefined>(
  path: string,
  body: Record<string, string | undefined>,
): Promise<AppResponse<T, M>> {
  const res = await api.patch<AppResponse<T, M>>(path, toFormUrlEncoded(body), {
    headers: { "Content-Type": "application/x-www-form-urlencoded" },
  });
  return res.data;
}

export async function patchFile<T = undefined, M = undefined>(
  path: string,
  file: File,
): Promise<AppResponse<T, M>> {
  const formData = new FormData();
  formData.append("file", file);
  const res = await api.patch<AppResponse<T, M>>(path, formData);
  return res.data;
}
