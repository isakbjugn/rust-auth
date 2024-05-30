import { BASE_API_URI } from '$lib/constants';

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
    return { error: true };
  }

  if (res.ok) {
    return { success: true };
  }
};