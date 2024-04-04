use askama::Template;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::Redirect;
use axum::routing::{delete, get, post};
use axum::{Form, Router};
use serde::Deserialize;
use sqlx::postgres::PgPool;
use tracing::error;

mod create;
mod delete;
mod list;
mod query;
mod redirect;

use crate::model::{CreateUrl, InputSingleText, UrlModel};

// SET UP API ENDPOINTS
pub fn api_routes() -> Router<crate::db::DBPool> {
    Router::new()
        .route("/hc", get(health_check))
        .route("/:input_string", get(redirect_url))
        .route("/url/create", post(create_url))
        .route("/url/list", get(list_urls))
        .route("/url/query", get(query_url))
        .route("/url/delete", delete(delete_url))
}

// Health Check
pub async fn health_check() -> StatusCode {
    StatusCode::OK
}

pub async fn create_url(
    State(state): State<PgPool>,
    Form(in_params): Form<CreateUrl>,
) -> Result<(StatusCode, OutputTemplate), (StatusCode, OutputTemplate)> {
    let input_params_clone = in_params.clone();
    let res = create::generate_url(state, in_params).await;
    match res {
        Ok(x) => Ok((
            StatusCode::OK,
            OutputTemplate {
                source_url: x.org_url,
                output_url: x.short_url,
                url_code: x.url_code,
            },
        )),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            OutputTemplate {
                source_url: input_params_clone.org_url,
                output_url: "".to_string(),
                url_code: 0,
            },
        )),
    }
}
#[derive(Deserialize, Template)]
#[template(path = "new_url.html")]
pub struct OutputTemplate {
    source_url: String,
    output_url: String,
    url_code: i32,
}

// List all available urls
#[derive(Template)]
#[template(path = "list_urls.html")]
pub struct ListURLTemplate {
    urls: Vec<UrlModel>,
}

pub async fn list_urls(
    State(state): State<PgPool>,
) -> Result<(StatusCode, ListURLTemplate), StatusCode> {
    match list::list_urls(state).await {
        Ok(urls) => Ok((StatusCode::OK, ListURLTemplate { urls })),
        Err(e) => {
            error!("{}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// Query a specific URL

#[derive(Template)]
#[template(path = "query_url.html")]
pub struct QueryURLTemplate {
    url: UrlModel,
}

#[derive(Debug, Deserialize)]
pub struct InputQueryText {
    pub url_id: String,
    pub url_code: i32,
}
pub async fn query_url(
    State(state): State<PgPool>,
    Query(in_query): Query<InputQueryText>,
) -> Result<(StatusCode, QueryURLTemplate), (StatusCode, QueryURLTemplate)> {
    match query::query(state, in_query).await {
        Ok(url) => Ok((StatusCode::OK, QueryURLTemplate { url })),
        Err(e) => {
            error!("{}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                QueryURLTemplate {
                    url: UrlModel::empty(),
                },
            ))
        }
    }
}

// Delete existing url
pub async fn delete_url(
    State(state): State<PgPool>,
    Query(in_query): Query<InputSingleText>,
) -> Result<StatusCode, StatusCode> {
    match delete::delete(state, in_query.input_string).await {
        Ok(status) => Ok(status),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

//Redirect URL
#[derive(Template)]
#[template(path = "not_found.html")]
pub struct RedirectErrorTemplate {}

pub async fn redirect_url(
    State(state): State<PgPool>,
    Path(in_path): Path<InputSingleText>,
) -> Result<Redirect, RedirectErrorTemplate> {
    match redirect::redirect(state, in_path.input_string).await {
        Ok(redirect_url) => Ok(Redirect::permanent(&redirect_url)),
        Err(_) => Err(RedirectErrorTemplate {}),
    }
}
