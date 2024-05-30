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
        padding: 16px 16px 16px 32px;
        margin: 8px auto 24px;
        background-color: #f7c6c7;
        width: 480px;
    }
</style>