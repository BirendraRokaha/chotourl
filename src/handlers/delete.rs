use axum::http::StatusCode;
use sqlx::PgPool;

pub async fn delete(
    state: PgPool,
    id_to_del: String,
) -> Result<StatusCode, Box<dyn std::error::Error>> {
    let res = sqlx::query(
        r#"
        DELETE FROM chotourls
        WHERE url_id = $1
        "#,
    )
    .bind(&id_to_del)
    .execute(&state)
    .await
    .map(|res| match res.rows_affected() {
        0 => StatusCode::NOT_FOUND,
        _ => StatusCode::OK,
    })?;
    println!("{:#?}", res);
    if res == StatusCode::NOT_FOUND {
        return Err(Box::new(std::fmt::Error));
    }
    Ok(res)
}
