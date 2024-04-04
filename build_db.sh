docker compose -f ./artifacts/docker-compose.yml up -d;
export DATABASE_URL=postgres://choto:chotopwd@localhost:5432; 
echo "Waiting for DB to start!!";
sleep 5;
cargo sqlx database create; 
cargo sqlx migrate run; 
cargo sqlx prepare;
echo "DONE!!";