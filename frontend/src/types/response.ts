export interface AppResponse<T = undefined, M = undefined> {
  code: number;
  status: string;
  message?: string;
  data?: T;
  meta?: M;
}

export interface AppError {
  code: number;
  status: string;
  message: string;
}
