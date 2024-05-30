<script>
    import { SUCCESS_EMOJI } from '$lib/emojis';
</script>
<div class="success">
    <span>
        {SUCCESS_EMOJI} <slot/>
    </span>
</div>

<style>
    .success {
        border-radius: 10px;
        padding: 16px 16px 16px 32px;
        margin: 8px auto 24px;
        background-color: #c2e7da;
        width: 480px;
    }
</style>