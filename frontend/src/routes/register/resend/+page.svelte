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