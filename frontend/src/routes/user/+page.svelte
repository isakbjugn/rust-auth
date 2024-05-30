<script lang="ts">
    import { page } from '$app/stores';
</script>

<svelte:head>
    {#if $page.data.user}
        <title>
            Auth - About {`${$page.data.user.first_name} ${$page.data.user.last_name}`} | Auth Systems with
                    SvelteKit
        </title>
    {/if}
</svelte:head>

{#if $page.data.user}
    <h1>Velkommen, {$page.data.user.first_name}!</h1>
    <div class="details">
        <p>Fornav: {$page.data.user.first_name}</p>
        <p>Etternavn: {$page.data.user.last_name}</p>
        <span>E-post: <a href="mailto:${$page.data.user.email}">{$page.data.user.email}</a></span>
        <p>Administrator: {$page.data.user.is_admin}</p>
    </div>
    <form method="POST" action="/api/logout">
        <button>Logg ut</button>
    </form>
{:else}
    <p>Laster...</p>
{/if}

<style>
    .details {
        min-width: 500px;
        margin: 0 auto;
    }
    form {
        min-width: 500px;
        margin: 0 auto;
    }
    button {
        padding: 0.5em;
    }
</style>