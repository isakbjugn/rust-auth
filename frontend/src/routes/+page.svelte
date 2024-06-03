<svelte:head>
	<title>Home</title>
	<meta name="description" content="Svelte demo app" />
</svelte:head>

<section>
	<h1>
		Velkommen til Rustning!
	</h1>
	<p>
		Rustning er en identitetstilbyder og autentiseringsløsning.
	</p>
	<a href="/login"><button>Logg inn</button></a>
	<p>eller</p>
	<a href="/register"><button>Registrer deg</button></a>
</section>

<style>
	section {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		flex: 0.3;
	}

	button {
		width: 400px;
	}

	h1 {
		width: 100%;
	}
</style>
