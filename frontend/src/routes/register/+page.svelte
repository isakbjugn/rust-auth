<script>
	/** @type {import('./$types').ActionData} */
	export let form;
	const conditionalFirstNameProps = form?.first_name ? { value: form.first_name } : {};
	const conditionalLastNameProps = form?.last_name ? { value: form.last_name } : {};
	const conditionalEmailProps = form?.email ? { value: form.email } : {};
</script>

<form method="POST">
	{#if form?.missing_fields}
		<p class="error">Please fill out all fields</p>
	{/if}
	<label>
		First name
		<input name="first_name" type="text" required {...conditionalFirstNameProps}>
	</label>
	<label>
		Last name
		<input name="last_name" type="text" required {...conditionalLastNameProps}>
	</label>
	{#if form?.conflicting_email}
		<p class="error">Email already exists</p>
	{/if}
	<label>
		Email
		<input name="email" type="email" required {...conditionalEmailProps}>
	</label>
	<label>
		Password
		<input name="password" type="password" required>
	</label>
	<button>Register</button>
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
	.error {
		color: red;
	}
	input:user-invalid {
		border-color: red;
	}
</style>