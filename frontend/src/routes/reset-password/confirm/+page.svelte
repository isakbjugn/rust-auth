<script>
  import SuccessMessage from '$lib/components/SuccessMessage.svelte';
  import ErrorMessage from '$lib/components/ErrorMessage.svelte';

  /** @type {import('./$types').ActionData} */
  export let form;
</script>

<section>
    {#if form?.missingToken}
        <div class="center">
            <ErrorMessage>Tilbakestillingslenken du brukte er ikke gyldig</ErrorMessage>
            <a href="/reset-password">Be om ny tilbakestillingslenke</a>
        </div>
    {:else if form?.success}
        <div class="center">
            <SuccessMessage>Du har nå byttet passord!</SuccessMessage>
        </div>
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
    section {
        max-width: 480px;
        margin: 0 auto;
    }
    .center {
        text-align: center;
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