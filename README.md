# Rust-auth

## Utvikling
### Start applikasjonen
Start docker-containeren i rot-katalogen:
```bash
docker-compose up -d
```

Start deretter selve applikasjonen:
```bash
cargo watch -q -c -w src/ -x run
```

Dersom du ikke ønsker at *cargo*-prosjektet automatisk skal bygge på nytt ved endringer, kan du kjøre:
```bash
cargo run
```

### Test applikasjonen
Katalogen `src/test` inneholder en rekke forberedte http-kall for brukerregistrering og innlogging. Disse kjøres direkte i IntelliJ.

Infokapsler (eng. *cookies*) ved disse http-kallene lagres i `.idea/httpRequests.http-client.cookies`, og kan redigeres der.

### Versjonskontroll
Bruk `git diff ':!*.lock'` for å vise lokale endringer uten å inkludere endringer i `Cargo.lock`.