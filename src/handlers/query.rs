use axum::http::StatusCode;
use sqlx::PgPool;

use crate::model::UrlModel;

use super::InputQueryText;

pub async fn query(state: PgPool, in_params: InputQueryText) -> Result<UrlModel, StatusCode> {
    let res =
        sqlx::query_as::<_, UrlModel>("SELECT * FROM chotourls where url_id=$1 AND url_code=$2")
            .bind(&in_params.url_id)
            .bind(&in_params.url_code)
            .fetch_one(&state)
            .await;

    match res {
        Ok(url) => {
            let _res = sqlx::query(
                r#"
                UPDATE chotourls
                SET updated_at=$2,
                WHERE url_id=$1
            "#,
            )
            .bind(&in_params.url_id)
            .bind(chrono::Utc::now())
            .execute(&state)
            .await;
            Ok(url)
        }
        Err(e) => {
            tracing::error!("{}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
