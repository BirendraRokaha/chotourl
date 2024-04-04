pub type DBPool = sqlx::postgres::PgPool;

pub async fn connect_to_database() -> DBPool {
    let database_url = "postgres://choto:chotopwd@localhost:5432";
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(10)
        .idle_timeout(std::time::Duration::from_secs(3))
        .connect(&database_url)
        .await
        .expect("Failed to connecto to the Database")
}

pub fn _map_db_err(err: sqlx::Error) -> (axum::http::StatusCode, String) {
    (
        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        err.to_string(),
    )
}
