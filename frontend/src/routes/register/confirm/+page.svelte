<script>
  import { page } from '$app/stores';
  import SuccessMessage from '$lib/components/SuccessMessage.svelte';
  import ErrorMessage from '$lib/components/ErrorMessage.svelte';
  /** @type {import('./$types').ActionData} */
</script>

<section>
    {#if $page.data.success}
        <SuccessMessage>Brukeren din er nå aktivert!</SuccessMessage>
    {:else if $page.data.missingToken}
        <ErrorMessage>Aktiveringslenken du brukte er ikke gyldig</ErrorMessage>
        <a href="/register/resend">Send ny aktiveringslenke</a>
    {:else if $page.data.error}
        <ErrorMessage>Noe gikk galt under aktiveringen</ErrorMessage>
        <a href="/register/resend">Send ny aktiveringslenke</a>
    {:else}
        <p>Aktiverer bruker...</p>
    {/if}
</section>

<style>
    section {
        text-align: center;
    }
</style>