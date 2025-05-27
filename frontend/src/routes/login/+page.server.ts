import { API_URL } from '$lib';
import { fail, redirect } from '@sveltejs/kit';

export const actions = {
  login: async ({ request, cookies }) => {
    const data = await request.formData();
    const username = data.get('username');
    const password = data.get('password');

    if (!username || !password) {
      return fail(400, {
        message: 'Username and password are required'
      });
    }

    try {
      const response = await fetch(`${API_URL}/auth/login`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ username, password })
      });

      if (!response.ok) {
        const message = await response.text();
        return fail(response.status, { message });
      }
      
      const resData = await response.json();
  

      cookies.set('jwt', resData.token, {
        path: '/',
        httpOnly: true,
        maxAge: 60 * 60 * 24, // 1 day
        sameSite: 'strict',
      });

    
      throw redirect(303, '/');
    } catch (error) {
       if ((error as any)?.status === 303) {
        throw error;
      }

      
      return fail(500, {
        message: 'An unexpected error occurred. Please try again later.'
      });
    }
  }
};