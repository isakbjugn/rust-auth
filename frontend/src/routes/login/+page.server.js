import { fail, redirect } from '@sveltejs/kit';
import { BASE_API_URI } from '$lib/constants';

/** @type {import('./$types').Actions} */
export const actions = {
  default: async ({ request, cookies,  }) => {
    const formData = await request.formData();
    const body = {
      email: formData.get('email'),
      password: formData.get('password')
    }

    const res = await fetch(`${BASE_API_URI}/login`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Accept: 'text/plain',
      },
      body: JSON.stringify(body)
    });

    if (!res.ok) {
      const response = await res.json();
      return fail(400, {error: response.error});
    }

    const { token } = await res.json();

    cookies.set('rust-auth', token, {
      httpOnly: true,
      sameSite: 'lax',
      path: '/',
      secure: true
    })

    throw redirect(302, '/user');
  }
};