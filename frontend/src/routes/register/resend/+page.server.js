import { BASE_API_URI } from '$lib/constants';
import { error, fail } from '@sveltejs/kit';

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
      error(500, {
        message: 'Noe gikk dessverre galt.'
      })
    }

    return { email: email, success: true  };
  }
};