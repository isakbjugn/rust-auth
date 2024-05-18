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
    <h1>User</h1>
    <p>{$page.data.user.first_name} {$page.data.user.last_name}</p>
{:else}
    <p>Loading...</p>
{/if}