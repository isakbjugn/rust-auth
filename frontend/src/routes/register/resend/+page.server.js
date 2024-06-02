import { BASE_API_URI } from '$lib/constants';
import { fail } from '@sveltejs/kit';

/** @type {import('./$types').Actions} */
export const actions = {
  default: async ({ request }) => {
    const formData = await request.formData();
    const email = formData.get('email');

    if (!email) {
      return fail(400, { email, missing_fields: true });
    }

    const body = {
      email: email,
    }

    const res = await fetch(`${BASE_API_URI}/users/register/resend`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(body)
    });

    if (!res.ok) {
      const response = await res.json();
      return fail(400, { error: response.error });
    }

    return { email: email, success: true  };
  }
};