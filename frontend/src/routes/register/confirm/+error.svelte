<script>
  import { page } from '$app/stores';
  import ErrorMessage from '$lib/components/ErrorMessage.svelte';
</script>

<section>
    {#if $page?.error}
        <ErrorMessage>{$page.error.message}</ErrorMessage>
        {#if $page.error.code === 'FORBIDDEN'}
            <a href="/register/resend">Be om ny aktiveringslenke</a>
        {/if}
    {/if}
</section>

<style>
    section {
        max-width: 480px;
        margin: 0 auto;
        text-align: center;
    }
</style>