import { redirect } from "@sveltejs/kit";

export async function POST({ cookies }) {
  cookies.delete('rust-auth', {
    path: '/',
    secure: true,
    sameSite: 'lax',
  })
  throw redirect(302, '/login');
}