# ChotoUrl (ShortURL)

Url shortner written in Rust

### Stack
1. Rust
    axum
    askama
    sqlx
2. Db
    postgres
3. Frontend
    tailwind, HTMX


## Build instructions

Needs cargo, sqlx-cli, docker/docker compose (optional for setting up DB)
```
git clone https://github.com/BirendraRokaha/chotourl.git
cd chotourl
cargo install sqlx-cli
# This will start a postgres db from 
./build_db.sh
cargo build --release
```
Server will start in port 8888

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