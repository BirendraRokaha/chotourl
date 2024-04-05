# ChotoUrl (ShortURL)

Url shortner written in Rust

### Stack
1. Rust: axum(server) askama(templates) sqlx(db)
2. Db: postgres
3. Frontend: tailwind, HTMX

<img src="https://github.com/BirendraRokaha/dcmrig/blob/main/artifacts/1.png">
<img src="https://github.com/BirendraRokaha/dcmrig/blob/main/artifacts/2.png">

## Build instructions

Needs cargo, sqlx-cli, docker/docker compose (optional for setting up DB)
```
git clone https://github.com/BirendraRokaha/chotourl.git
cd chotourl
# Check .env to set up PORT, DOMAIN, DB_URL
cargo install sqlx-cli
# This will start a postgres db from 
./build_db.sh
cargo build --release
./target/release/chotourl
```

To set up external DB 
```
export DATABASE_URL=postgres://UNAME:PWD@URL:PORT;
cargo sqlx database create; 
cargo sqlx migrate run; 
cargo sqlx prepare;
```

```
# Running Tailwind
pnpm dlx tailwindcss -i styles/tailwind.css -o assets/main.css --watch
```

## Features
1. Generate short URL
2. Add a custom phrase/alias
3. Track Url using UrlID and a 6 digit code
4. Delete Url

## API 
```
GET / IndexPage
GET /hc HealthCheck
GET /:url_id Redirects to the long URL if ID exists
POST /url/create Create a new URL from the form entry [LONG_URL: REQ, ALIAS: OPTIONAL]
GET /url/query Get stats for URL [URL_ID: REQ, ACCESS_CODE: REQ]
DELETE / url/delete Delete URL record
```


## TODO
1. Set up simple admin dashboard
2. Set up expirey for url records
3. Set up multi short urls for the same long url