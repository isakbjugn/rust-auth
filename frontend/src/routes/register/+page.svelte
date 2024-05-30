<script>
	/** @type {import('./$types').ActionData} */
	export let form;
	const conditionalFirstNameProps = form?.first_name ? { value: form.first_name } : {};
	const conditionalLastNameProps = form?.last_name ? { value: form.last_name } : {};
	const conditionalEmailProps = form?.email ? { value: form.email } : {};
</script>

<section>
	{#if form?.success}
		<div class="success">
			<p>En aktiveringslenke er nå sendt på e-post</p>
		</div>
	{/if}
	<form method="POST">
		<fieldset>
			<legend>Registrer deg</legend>
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
		</fieldset>
	</form>
</section>

<style>
	.success {
		border-radius: 10px;
		padding: 10px 10px 10px 36px;
		margin: 10px auto;
		background-color: #c2e7da;
		width: 480px;
	}
	fieldset {
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
	.error {
		color: red;
	}
	input:user-invalid {
		border-color: red;
	}
</style>