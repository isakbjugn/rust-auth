<script>
	import ErrorMessage from '$lib/components/ErrorMessage.svelte';

	/** @type {import('./$types').ActionData} */
	export let form;
	const conditionalEmailProps = form?.email ? { value: form.email } : {};
</script>

<form method="POST">
	{#if form?.error}
		<ErrorMessage>{form?.error}</ErrorMessage>
	{/if}
	<label>
		Email
		<input name="email" type="email" {...conditionalEmailProps}>
	</label>
	<label>
		Password
		<input name="password" type="password">
	</label>
	<button>Log in</button>
</form>

<style>
	form {
		display: grid;
		grid-gap: 1em;
		width: 480px;
		max-width: 480px;
		margin: 0 auto;
	}
	@media screen and (max-width: 512px) {
		form {
			width: unset;
			max-width: 100%;
			margin: 0 16px;
		}
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