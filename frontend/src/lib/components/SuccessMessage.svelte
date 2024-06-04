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
        padding: 16px;
        background-color: #c2e7da;
        margin-bottom: 16px;
    }
</style>