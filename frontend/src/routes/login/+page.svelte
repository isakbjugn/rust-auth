<script lang="ts">
	import { goto } from '$app/navigation';
	import { BASE_API_URI } from "$lib/constants";
	let email = '',
		password = '';
	const handleLogin = async () => {
		const body = { email, password };
		fetch(`${BASE_API_URI}/login/local`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
				Accept: 'text/plain',
			},
			credentials: 'include',
			body: JSON.stringify(body)
		}).then(async (response) => {
			if (response.ok) {
				console.log("Response: ", response);
				goto('/user');
			} else {
				console.error(response);
			}
		}).catch((error) => {
			console.error(error);
		});
	};
</script>

<form on:submit={handleLogin}>
	<label>
		Email
		<input bind:value={email} name="email" type="email">
	</label>
	<label>
		Password
		<input bind:value={password} name="password" type="password">
	</label>
	<button type="submit">Log in</button>
</form>

<style>
	form {
		display: grid;
		grid-gap: 1em;
		min-width: 500px;
		margin: 0 auto;
	}
	label {
		display: grid;
		grid-gap: 0.5em;
	}
	input {
		padding: 0.5em;
	}
	button {
		padding: 0.5em;
	}
</style>