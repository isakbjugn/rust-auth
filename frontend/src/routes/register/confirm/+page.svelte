<script>
  import { page } from '$app/stores';
  import SuccessMessage from '$lib/components/SuccessMessage.svelte';
  import ErrorMessage from '$lib/components/ErrorMessage.svelte';
</script>

<section>
    {#if $page.data.success}
        <SuccessMessage>Brukeren din er nå aktivert!</SuccessMessage>
    {:else if $page.data.missingToken}
        <ErrorMessage>Aktiveringslenken du brukte er ikke gyldig</ErrorMessage>
        <a href="/register/resend">Send ny aktiveringslenke</a>
    {:else}
        <p>Aktiverer bruker...</p>
    {/if}
</section>

<style>
    section {
        max-width: 480px;
        margin: 0 auto;
        text-align: center;
    }
</style>