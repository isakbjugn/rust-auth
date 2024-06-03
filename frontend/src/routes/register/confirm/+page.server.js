import { BASE_API_URI } from '$lib/constants';
import { error } from '@sveltejs/kit';

/** @type {import('./$types').PageServerLoad} */
export const load = async ({ request }) => {
  const token = new URL(request.url).searchParams.get('token');
  if (!token) {
    return { missingToken: true };
  }
  const res = await fetch(`${BASE_API_URI}/users/register/confirm`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      Accept: 'text/plain',
    },
    body: JSON.stringify({ token })
  });

  if (!res.ok) {
    if (res.status === 403) {
      return error(403, {
        message: 'Denne lenken kan ikke brukes til å aktivere brukeren din',
        code: 'FORBIDDEN'
      })
    } else {
      return error(500, {
        message: 'Noe gikk dessverre galt.',
        code: 'INTERNAL_SERVER_ERROR'
      })
    }
  }

  if (res.ok) {
    return { success: true };
  }
};