import { BASE_API_URI } from '$lib/constants';
import type { User } from '$lib/types';
import type { Handle } from '@sveltejs/kit';

export const handle: Handle = async ({ event, resolve }) => {
  if (event.locals.user) {
    // if there is already a user  in session load page as normal
    return resolve(event);
  }
  // get cookies from browser
  const session = event.cookies.get('rust-auth');

  if (!session) {
    // if there is no session load page as normal
    return resolve(event);
  }

  // find the user based on the session
  const res = await event.fetch(`${BASE_API_URI}/users`, {
    credentials: 'include',
    headers: {
      Authorization: `Bearer ${session}`
    }
  });

  if (!res.ok) {
    // if there is no session load page as normal
    return resolve(event);
  }

  // if `user` exists set `events.local`
  event.locals.user = (await res.json()) as User;

  // load page as normal
  return resolve(event);
};
