import { API_URL } from '$lib';
import { fail, redirect } from '@sveltejs/kit';

export const actions = {
  register: async ({ request, cookies }) => {
    const data = await request.formData();
    const username = data.get('username');
    const email = data.get('email');
    const password = data.get('password');
    const confirmPassword = data.get('confirmPassword');

    if (!username || !email || !password || !confirmPassword) {
      return fail(400, {
        message: 'All fields are required'
      });
    }

    if (password !== confirmPassword) {
      return fail(400, {
        message: 'Passwords do not match'
      });
    }

    const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
    if (!emailRegex.test(email.toString())) {
      return fail(400, {
        message: 'Please enter a valid email address'
      });
    }

    console.log('Registering user:', { username, email, password });

    try {
      const response = await fetch(`${API_URL}/auth/register`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ username, email, password })
      });


      if (!response.ok) {
        const message = await response.text();
        return fail(response.status, { message });
      }
            
      const resData = await response.json();
      console.log(resData);
  
      


    
      throw redirect(303, '/login');
    } catch (error) {
       if ((error as any)?.status === 303) {
        throw error;
      }

      console.error('Error during registration:', error);
      
      return fail(500, {
        message: 'An unexpected error occurred. Please try again later.'
      });
    }
  }
};