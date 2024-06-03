import { BASE_API_URI } from '$lib/constants';
import { error, fail } from '@sveltejs/kit';

/** @type {import('./$types').Actions} */
export const actions = {
  default: async ({ request }) => {
    const token = new URL(request.url).searchParams.get('token');
    if (!token) {
      return { missingToken: true };
    }
    const formData = await request.formData();
    const new_password = formData.get('new_password');

    if (!new_password) {
      return fail(400, { missing_fields: true });
    }

    const body = {
      token: token,
      new_password: new_password,
    }

    const res = await fetch(`${BASE_API_URI}/users/reset-password/confirm`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(body)
    });

    if (!res.ok) {
      if (res.status === 403) {
        return error(403, {
          message: 'Denne lenken kan ikke brukes til å tilbakestille passordet ditt',
          code: 'FORBIDDEN'
        })
      } else {
        return error(500, {
          message: 'Noe gikk dessverre galt.',
          code: 'INTERNAL_SERVER_ERROR'
        })
      }
    }

    return { success: true  };
  }
};