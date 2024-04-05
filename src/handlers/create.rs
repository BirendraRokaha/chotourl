use crate::model::{CreateUrl, UrlModel};
use dotenv::dotenv;
use rand::{distributions::Alphanumeric, Rng};
use sqlx::PgPool;
use std::fmt::Error;
use url::Url;

pub async fn generate_url(
    state: PgPool,
    params: CreateUrl,
) -> Result<UrlModel, Box<dyn std::error::Error>> {
    let (uniq_id, cust_phrase) = if params.cust_phrase.as_deref().unwrap_or("").len() == 0 {
        (gen_id(), "N/A".to_string())
    } else {
        let cust_phrase = params.cust_phrase.as_deref().unwrap().to_string();
        (cust_phrase.clone(), cust_phrase.clone())
    };

    let validated_url: String = validate_url(params.org_url)?;
    let uniq_id = if !check_uniq_id(&state, uniq_id.clone()).await {
        uniq_id
    } else {
        loop {
            let new_id = gen_id();
            if !check_uniq_id(&state, new_id.clone()).await {
                break new_id;
            } else {
                continue;
            }
        }
    };

    let url = generate_url_obj(uniq_id, validated_url, cust_phrase);

    let _res = sqlx::query(
        r#"
         INSERT INTO chotourls 
        (url_id, org_url, short_url, cust_phrase, inserted_at, updated_at,visits, url_code)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)       
        "#,
    )
    .bind(&url.url_id)
    .bind(&url.org_url)
    .bind(&url.short_url)
    .bind(&url.cust_phrase)
    .bind(&url.inserted_at)
    .bind(&url.updated_at)
    .bind(&url.visits)
    .bind(&url.url_code)
    .execute(&state)
    .await?;

    Ok(url)
}

fn gen_id() -> String {
    let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(5)
        .map(char::from)
        .collect();
    s
}

async fn check_uniq_id(state: &PgPool, uniq_id: String) -> bool {
    sqlx::query("SELECT url_id FROM chotourls WHERE url_id = $1")
        .bind(&uniq_id)
        .fetch_optional(state)
        .await
        .unwrap()
        .is_some()
}

fn generate_url_obj(url_id: String, input_url: String, cust_phrase: String) -> UrlModel {
    dotenv().ok();
    let domain = std::env::var("DOMAIN").unwrap_or("192.168.0.240".to_string());
    let time_now = chrono::Utc::now();
    let short_url = format!("http://{}/{}", domain, url_id.clone());
    UrlModel {
        url_id: url_id.clone(),
        org_url: input_url,
        short_url: short_url,
        cust_phrase: cust_phrase,
        inserted_at: time_now.clone(),
        updated_at: time_now.clone(),
        visits: 0,
        url_code: rand::thread_rng().gen_range(100000..=999999),
    }
}

fn validate_url(in_url: String) -> Result<String, Box<dyn std::error::Error>> {
    if in_url.is_empty() || in_url.len() < 5 {
        return Err(Box::new(Error));
    };

    let mut source_url = in_url.clone();
    if &source_url[..4] != "http" {
        source_url = format!("https://{source_url}")
    }

    Ok(Url::parse(&source_url)?.to_string())
}
