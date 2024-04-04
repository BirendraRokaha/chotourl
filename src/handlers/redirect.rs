use axum::http::StatusCode;
use sqlx::PgPool;

use crate::model::UrlModel;

pub async fn redirect(state: PgPool, id_to_query: String) -> Result<String, StatusCode> {
    let res = sqlx::query_as::<_, UrlModel>("SELECT * FROM chotourls where url_id=$1")
        .bind(&id_to_query)
        .fetch_one(&state)
        .await;
    match res {
        Ok(url) => {
            let _res = sqlx::query(
                r#"
                UPDATE chotourls
                SET visits = visits + 1,
                    updated_at=$2
                WHERE url_id=$1
            "#,
            )
            .bind(&id_to_query)
            .bind(chrono::Utc::now())
            .execute(&state)
            .await;
            Ok(url.org_url)
        }
        Err(e) => {
            tracing::error!("{}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
