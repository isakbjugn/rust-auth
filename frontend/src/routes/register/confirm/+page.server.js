import { redirect } from '@sveltejs/kit';
import { BASE_API_URI } from '$lib/constants';

/** @type {import('./$types').PageServerLoad} */
export const load = async ({ request }) => {
  const token = new URL(request.url).searchParams.get('token');
  const res = await fetch(`${BASE_API_URI}/users/register/confirm`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      Accept: 'text/plain',
    },
    body: JSON.stringify({ token })
  });

  if (res.ok) {
    throw redirect(303, `/register/confirmed`);
  }
};