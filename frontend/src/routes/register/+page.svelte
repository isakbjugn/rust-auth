<script>
	import SuccessMessage from '$lib/components/SuccessMessage.svelte';
	import ErrorMessage from '$lib/components/ErrorMessage.svelte';

	/** @type {import('./$types').ActionData} */
	export let form;
	const conditionalFirstNameProps = form?.first_name ? { value: form.first_name } : {};
	const conditionalLastNameProps = form?.last_name ? { value: form.last_name } : {};
	const conditionalEmailProps = form?.email ? { value: form.email } : {};
</script>

<section>
	{#if form?.success}
		<SuccessMessage>En aktiveringslenke er nå sendt på e-post</SuccessMessage>
	{/if}
	{#if form?.error}
		<ErrorMessage>Noe gikk galt!</ErrorMessage>
	{/if}
	<form method="POST">
		{#if form?.missing_fields}
			<p class="error">Vennligst fyll ut alle felt</p>
		{/if}
		<label>
			Fornavn
			<input name="first_name" type="text" required {...conditionalFirstNameProps}>
		</label>
		<label>
			Etternavn
			<input name="last_name" type="text" required {...conditionalLastNameProps}>
		</label>
		<label>
			E-postadresse
			<input name="email" type="email" required {...conditionalEmailProps}>
		</label>
		<label>
			Passord
			<input name="password" type="password" required>
		</label>
		<button>Registrer</button>
	</form>
	<p class="remark"> Har du allerede en konto? <a href="/login">Logg inn</a></p>
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
	.error {
		color: red;
	}
	input:user-invalid {
		border-color: red;
	}
</style>