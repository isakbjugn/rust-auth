<script>
  import SuccessMessage from '$lib/components/SuccessMessage.svelte';
  import ErrorMessage from '$lib/components/ErrorMessage.svelte';

  /** @type {import('./$types').ActionData} */
  export let form;
</script>

<section>
    {#if form?.success}
        <SuccessMessage>Du har nå byttet passord!</SuccessMessage>
    {:else if form?.missingToken}
        <ErrorMessage>Tilbakestillingslenken du brukte er ikke gyldig</ErrorMessage>
        <a href="/register/resend">Send ny aktiveringslenke</a>

    {:else}
        <form method="post">
            <label>
                Nytt passord
                <input type="password" name="new_password" required>
            </label>
            <button type="submit">Bytt passord</button>
        </form>
    {/if}
</section>

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
    input:user-invalid {
        border-color: red;
    }
</style>