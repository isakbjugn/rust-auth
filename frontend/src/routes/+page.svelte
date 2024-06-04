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
		max-width: 480px;
		margin: 0 auto;
		text-align: center;
	}
	a {
		min-width: 100%;
	}
	button {
		min-width: 100%;
	}
	h1 {
		width: 100%;
	}
</style>
