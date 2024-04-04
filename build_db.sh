cd artifacts;
docker compose up -d ;
cd ..;
export DATABASE_URL=postgres://choto:chotopwd@localhost:5432 ; 
echo "Waiting for DB to start!!"
sleep 5;
cargo sqlx database create ; 
cargo sqlx migrate run ; 
cargo sqlx prepare;
echo "DONE!!"