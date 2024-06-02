<script>
  import { page } from '$app/stores';
  import ErrorMessage from "$lib/components/ErrorMessage.svelte";
</script>

<ErrorMessage>{$page.error?.message}</ErrorMessage>