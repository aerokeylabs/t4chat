import { useKeys } from '@/composables/keys';
import type { CancelMessageRequest, CancelMessageResponse } from '@/lib/types';
import { SSE, type SSEHeaders } from 'sse.js';

export function getApiUrl(path: string): string {
  const baseUrl = import.meta.env.VITE_API_URL;

  if (!baseUrl) {
    throw new Error('VITE_API_URL is not defined');
  }

  return `${baseUrl}/${path}`;
}

export async function apiPost<TRes, TReq>(path: string, body: TReq): Promise<TRes> {
  const url = getApiUrl(path);
  const res = await fetch(url, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(body),
  });

  if (!res.ok) {
    throw new Error(
      `API request failed with status ${res.status}: ${res.body ? await res.text() : 'No response body'}`,
    );
  }

  return (await res.json()) as TRes;
}

export async function apiGet<T>(path: string): Promise<T> {
  const url = getApiUrl(path);
  const res = await fetch(url, {
    method: 'GET',
    headers: {
      'Content-Type': 'application/json',
    },
  });

  if (!res.ok) {
    throw new Error(
      `API request failed with status ${res.status}: ${res.body ? await res.text() : 'No response body'}`,
    );
  }

  return (await res.json()) as T;
}

export function apiPostSse<TReq>(path: string, body: TReq): SSE {
  const keys = useKeys();
  const headers: SSEHeaders = {
    'Content-Type': 'application/json',
  };

  let openrouter = keys.openrouter.trim();
  if (openrouter !== '') {
    headers['X-OpenRouter-Key'] = openrouter;
  }

  const url = getApiUrl(path);
  return new SSE(url, {
    headers,
    payload: JSON.stringify(body),
    method: 'POST',
  });
}

export async function cancelMessage(threadId: string) {
  return apiPost<CancelMessageResponse, CancelMessageRequest>('message/cancel', { threadId });
}
