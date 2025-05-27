import { API_URL } from '$lib';
import type { RequestHandler } from './$types';

export const GET: RequestHandler = async ({ cookies, params, fetch }) => {
  const token = cookies.get('jwt');
  if (!token) {
    return new Response('Unauthorized', { status: 401 });
  }

  const upstream = await fetch(`${API_URL}/image/${params.id}`, {
    headers: {
      Authorization: `Bearer ${token}`
    }
  });

  if (!upstream.ok) {
    return new Response('Not found', { status: upstream.status });
  }

  const contentType = upstream.headers.get('content-type') ?? 'application/octet-stream';
  const buffer = await upstream.arrayBuffer();
  return new Response(buffer, {
    headers: { 'Content-Type': contentType }
  });
};
