<script>
  import { page } from '$app/stores';
  import ErrorMessage from '$lib/components/ErrorMessage.svelte';
</script>

{#if $page.data.error}
    <ErrorMessage>{$page.data.error}</ErrorMessage>
    {#if $page.data.error.code === 'FORBIDDEN'}
        <a href="/register/resend">Be om ny aktiveringslenke</a>
    {/if}
{/if}