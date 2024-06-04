<script>
    import { ERROR_EMOJI } from '$lib/emojis';
</script>

<div class="error">
    <span>
        {ERROR_EMOJI} <slot/>
    </span>
</div>

<style>
    .error {
        border-radius: 10px;
        padding: 16px;
        background-color: #f7c6c7;
        margin-bottom: 16px;
    }
</style>