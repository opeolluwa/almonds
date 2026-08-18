import type { FetchError } from "ofetch";
import { $fetch } from "ofetch";

export interface SignupRequest {
  email: string;
  password: string;
}

export interface LoginRequest {
  email: string;
  password: string;
}

export interface ForgottenPasswordRequest {
  email: string;
}

export interface SetNewPasswordRequest {
  password: string;
  confirmPassword: string;
}

export interface VerifyOtpRequest {
  otp: string;
}

export interface AcceptInvitationRequest {
  token: string;
}

export interface ResendOtpRequest {
  flow: string;
}

export interface LoginResponse {
  message?: string;
  accessToken: string;
  refreshToken: string;
  iat: number;
  exp: number;
  refreshTokenExp: number;
  refreshTokenIat: number;
}

export interface TokenResponse {
  message?: string;
  token: string;
}

interface ApiErrorBody {
  message?: string;
}

export function useAuthApi() {
  const config = useRuntimeConfig();
  const authStore = useAuthStore();

  const baseUrl = computed(() =>
    (config.public.serverUrl as string)
      .replace(/\/+$/, "")
      .replace(/\/orchard$/, ""),
  );

  async function post<T>(
    path: string,
    body?: unknown,
    requestOptions: { token?: string } = {},
  ): Promise<T> {
    const headers: Record<string, string> = {};
    const token = requestOptions.token ?? authStore.accessToken;
    if (token) headers.Authorization = `Bearer ${token}`;

    try {
      return await $fetch<T>(path, {
        baseURL: baseUrl.value,
        method: "POST",
        body,
        headers,
      });
    } catch (error) {
      const err = error as FetchError;
      const message = (err.data as ApiErrorBody | undefined)?.message;
      if (message) throw new Error(message);
      throw new Error("Something went wrong. Please try again.");
    }
  }

  return {
    baseUrl,
    post,
    signup: (req: SignupRequest) => post<TokenResponse>("/auth/signup", req),
    login: (req: LoginRequest) => post<LoginResponse>("/auth/login", req),
    forgottenPassword: (req: ForgottenPasswordRequest) =>
      post<TokenResponse>("/auth/forgotten-password", req),
    verifyAccount: (req: VerifyOtpRequest, token: string) =>
      post<TokenResponse>("/auth/verify-account", req, { token }),
    verifyResetOtp: (req: VerifyOtpRequest, token: string) =>
      post<TokenResponse>("/auth/verify", req, { token }),
    setNewPassword: (req: SetNewPasswordRequest, token: string) =>
      post<{ message?: string }>("/auth/reset-password", req, { token }),
    acceptInvitation: (req: AcceptInvitationRequest) =>
      post<{ message?: string }>("/invitations/accept", req),
    resendOtp: (req: ResendOtpRequest, token: string) =>
      post<TokenResponse>("/auth/resend-otp", req, { token }),
  };
}
