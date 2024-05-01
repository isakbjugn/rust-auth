<main>
    <h1>Rust-auth</h1>
    <img src="/rustning.png" alt="Skjold med en krabbe som skjoldmerke" width="200" height="200">
    <div>
        <h2>Logg inn</h2>
        <form method="POST">
            <label>
                E-post
                <input name="email" type="email" />
            </label>
            <label>
                Passord
                <input name="password" type="password" />
            </label>
            <button>Logg inn</button>
        </form>
    </div>
    <p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>
</main>

<style>
    main {
        max-width: 490px;
        border-radius: 1rem;
        margin: 0.5rem;
        background-color: #709fae;
        padding: 1.25rem;
        font-size: 18px;
    }
    h1 {
        text-align: center;
    }
    img {
        display: block;
        margin: 0 auto;
    }
    @media screen and (min-width: 34rem) {
        main {
            margin: 1rem auto;
        }
    }
</style>