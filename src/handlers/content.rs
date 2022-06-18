use actix_web::{
    web,
    HttpResponse,
    error,
    http::{
        StatusCode,
        header
    }
};
use serde::{
    Deserialize
};
use crate::db::{
    models::{
        ContentType,
        FullContent,
        NewFullContent
    },
    ops::content_ops,
    DbPool,
};
use super::errors::{
    ContentError
};

// ######################################################################################################
// ------------------------------------------------------------------------------------------------------
// ######################################################################################################
// ######################################################################################################
// ------------------------------------------------------------------------------------------------------
// ######################################################################################################

#[derive(Deserialize, Debug)]
pub struct ContentSlug {
    slug: String
}

#[derive(Deserialize, Debug)]
pub struct PageInfo {
    content_per_page: i64,
    page: Option<i32>,
}

// ######################################################################################################
// ------------------------------------------------------------------------------------------------------
// ######################################################################################################

pub async fn view_content(
    db_pool: web::Data<DbPool>, 
    slug_requested: web::Path<ContentSlug>
) -> Result<web::Json<FullContent>, ContentError> {
    let fetched_content = web::block(move || {
        let conn = db_pool.get()?;
        content_ops::view_content(
            &conn,
            &slug_requested.into_inner().slug
        )
    })
    .await??;
    Ok(web::Json(fetched_content))
}

pub async fn recent_content(
    db_pool: web::Data<DbPool>, 
    content_type_requested: web::Path<ContentType>,
    page_info: web::Query<PageInfo>
) -> Result<web::Json<Vec<FullContent>>, ContentError> {
    let recent_fetched_content = web::block(move || {
        let conn = db_pool.get()?;
        content_ops::view_recent_content(
            &conn,
            content_type_requested.into_inner(),
            page_info.into_inner().content_per_page
        )
    })
    .await??;
    Ok(web::Json(recent_fetched_content))
}

pub async fn update_content(
    db_pool: web::Data<DbPool>, 
    content_info: web::Path<ContentSlug>,
    // update_info: Json<>
) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub async fn delete_content(
    db_pool: web::Data<DbPool>, 
    delete_slug: web::Path<ContentSlug>,
) -> Result<HttpResponse, ContentError> {
    // Returns the number of rows deleted
    let rows_deleted = web::block(move || {
        let conn = db_pool.get()?;
        content_ops::delete_content(
            &conn,
            delete_slug.into_inner().slug
        )
    })
    .await??;
    Ok(HttpResponse::Ok().json(format!(r#"
    {{
        "rows_deleted": "{}"
    }}
    "#, rows_deleted)))
}

pub async fn add_content(
    db_pool: web::Data<DbPool>, 
    add_info: web::Json<NewFullContent>
) -> Result<HttpResponse, ContentError> {
    web::block(move || {
        let conn = db_pool.get()?;
        content_ops::add_content(
            &conn,
            add_info.into_inner()
        )
    })
    .await??;
    Ok(HttpResponse::Ok().finish())
}
