<script>
  import SuccessMessage from '$lib/components/SuccessMessage.svelte';

  /** @type {import('./$types').ActionData} */
  export let form;
</script>

<section>
    {#if form?.success}
        <SuccessMessage>En ny aktiveringslenke er sendt til {form.email}!</SuccessMessage>
    {/if}
    <form method="POST">
        <label>
            E-postadresse
            <input name="email" type="email" required>
        </label>
        <button type="submit">Send ny aktiveringslenke</button>
    </form>
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
    input:user-invalid {
        border-color: red;
    }
</style>