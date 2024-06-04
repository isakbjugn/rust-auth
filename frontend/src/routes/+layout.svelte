<script>
	import Header from './Header.svelte';
	import './styles.css';
	import rustning from '$lib/images/rustning.png';
</script>

<div class="app">
	<Header/>
	<picture>
		<img src={rustning} alt="Rustning" height="200px" />
	</picture>

	<main>
		<slot/>
	</main>

	<footer>
		<p>Sjekk ut <a href="https://github.com/isakbjugn/rust-auth">isakbjugn/rust-auth</a> om du vil se nærmere på koden</p>
	</footer>
</div>

<style>
	.app {
		display: flex;
		flex-direction: column;
		min-height: 100vh;
	}

	main {
		flex: 1;
		padding: 1rem;
		width: 100%;
		max-width: 64rem;
		margin: 0 auto;
		box-sizing: border-box;
	}

	picture {
		display: flex;
		justify-content: center;
		align-items: center;
	}

	footer {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		padding: 12px;
		text-align: center;
	}

	footer a {
		font-weight: bold;
	}

	@media (min-width: 480px) {
		footer {
			padding: 12px 0;
		}
	}
</style>
