import { fail, redirect } from '@sveltejs/kit';
import { BASE_API_URI } from '$lib/constants';

/** @type {import('./$types').Actions} */
export const actions = {
  default: async ({ request, cookies,  }) => {
    const formData = await request.formData();
    const email = formData.get('email');
    const password = formData.get('password');
    const first_name = formData.get('first_name');
    const last_name = formData.get('last_name');

    const body = {
      email: email,
      password: password,
      first_name: first_name,
      last_name: last_name,
    }

    if (!email || !password || !first_name || !last_name) {
      return fail(400, { first_name, last_name, email, missing_fields: true });
    }

    const res = await fetch(`${BASE_API_URI}/users/register`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Accept: 'text/plain',
      },
      body: JSON.stringify(body)
    });

    if (res.status === 409) {
      return fail(409, { first_name, last_name, email, conflicting_email: true });
    }

    if (!res.ok) {
      const response = await res.json();
      return fail(400, { error: response.error });
    }

    throw redirect(303, `/register/confirm`);
  }
};