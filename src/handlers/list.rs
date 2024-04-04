use crate::model::UrlModel;
use sqlx::PgPool;

pub async fn list_urls(state: PgPool) -> Result<Vec<UrlModel>, Box<dyn std::error::Error>> {
    let req = sqlx::query_as::<_, UrlModel>("SELECT * FROM chotourls")
        .fetch_all(&state)
        .await
        .unwrap();
    // println!("{:#?}", req);
    Ok(req)
}
