
use leptos::{*};

#[cfg(feature = "ssr")]
use actix_web::web::Data;
#[cfg(feature = "ssr")]
use sqlx::{Pool, Sqlite};

#[cfg(feature = "ssr")]
use leptos_actix::extract;

use crate::model::blog_post::Post;


#[server(UpsertPost, "/api")]
pub async fn upsert_post(
    id: Option<String>,
    dt: String,
    image_url: String,
    title: String,
    text: String,
) -> Result<String, ServerFnError> {
    let pool: Data<Pool<Sqlite>> = extract().await?;
    let pool = pool.into_inner();
    use uuid::Uuid;
    let id = id.unwrap_or_else(|| Uuid::new_v4().to_string());
    sqlx::query("INSERT INTO post VALUES ($1, $2, $3, $4, $5) 
                ON CONFLICT(id) DO UPDATE SET dt = excluded.dt, 
                image_url = excluded.image_url, title = excluded.title, text = excluded.text")
        .bind(&id)
        .bind(&dt)
        .bind(&image_url)
        .bind(&title)
        .bind(&text)
        .execute(&*pool)
        .await?;
    log::info!("upserted post with id: {}", &id);
    Ok(id.to_string())
}

#[server(GetPost, "/api")]
pub async fn get_post(id: String) -> Result<Post, ServerFnError> {
    let pool: Data<Pool<Sqlite>> = extract().await?;
    let pool = pool.into_inner();

    let res: Post = sqlx::query_as("SELECT * FROM post WHERE id = $1")
        .bind(&id)
        .fetch_one(&*pool)
        .await
        .map_err(|_| -> ServerFnError {ServerFnError::ServerError("error getting post".to_string())})?;
    
    Ok(res)
}

#[server(GetPreviews, "/api")]
pub async fn get_previews(
    _oldest: Option<String>,
    _newest: Option<String>,
    preview_length: u8,
    page_size: u8,
) -> Result<Vec<Post>, ServerFnError> {
    let pool: Data<Pool<Sqlite>> = extract().await?;
    let pool = pool.into_inner();
    let res: Vec<Post> = sqlx::query_as(
        "SELECT 
                id, dt, image_url, title,
                CASE WHEN LENGTH(text) > $1 THEN SUBSTR(text, $1, ?) || '...'
                    ELSE text
                END AS text
            FROM post
            ORDER BY dt DESC
            LIMIT $2",
    )
    .bind(preview_length)
    .bind(page_size)
    .fetch_all(&*pool)
    .await
    .map_err(|_| -> ServerFnError {ServerFnError::ServerError("error getting post".to_string())})?;

    Ok(res)
}

#[server(DeletePost, "/api")]
pub async fn delete_post(id: String) -> Result<(), ServerFnError> {
    let pool: Data<Pool<Sqlite>> = extract().await?;
    let pool = pool.into_inner();
    sqlx::query("DELETE FROM post WHERE id = $1")
        .bind(&id)
        .execute(&*pool)
        .await?;
    Ok(())
}