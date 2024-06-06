<script>
  import { page } from '$app/stores';
  import ErrorMessage from '$lib/components/ErrorMessage.svelte';
</script>

<section>
    <ErrorMessage>{$page.error?.message}</ErrorMessage>
</section>

<style>
    section {
        max-width: 480px;
        margin: 0 auto;
        text-align: center;
    }
</style>