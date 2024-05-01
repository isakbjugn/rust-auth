const backendUrl = 'https://localhost:4000'

/** @type {import('./$types').Actions} */
export const actions = {
  default: async ({ request }) => {
    const data = await request.formData()
    fetch(`${backendUrl}/users/login`,  {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({
        email: data.get('email'),
        password: data.get('password')
      })
    })
  },
};