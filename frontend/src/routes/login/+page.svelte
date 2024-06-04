<script>
	import ErrorMessage from '$lib/components/ErrorMessage.svelte';

	/** @type {import('./$types').ActionData} */
	export let form;
	const conditionalEmailProps = form?.email ? { value: form.email } : {};
</script>

<section>
	<form method="POST">
		{#if form?.error}
			<ErrorMessage>{form?.error}</ErrorMessage>
		{/if}
		<label>
			E-post
			<input name="email" type="email" {...conditionalEmailProps}>
		</label>
		<label>
			Passord
			<input name="password" type="password">
		</label>
		<button>Logg in</button>
	</form>
	<p class="remark"> Har du glemt passordet ditt? <a href="/reset-password">Tilbakestill</a></p>
</section>

<style>
	section {
		max-width: 480px;
		margin: 0 auto;
	}
	form {
		display: grid;
		grid-gap: 1em;
	}
	.remark {
		text-align: center;
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