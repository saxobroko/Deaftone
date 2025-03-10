use std::collections::HashMap;

use axum::{
    body::{boxed, Body, BoxBody, Full},
    extract::{Path, State},
    http::{header, Request, Response, StatusCode},
    Json,
};

use include_dir::{include_dir, Dir};
use sea_orm::EntityTrait;
use serde::Serialize;
use tower::ServiceExt;
use tower_http::services::ServeFile;

use crate::{services, AppState};

static ASSETS: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/resources");
#[allow(non_snake_case)]
#[derive(Serialize)]
pub struct AlbumResponse {
    id: String,
    name: String,
    artist: String,
    artistId: String,
    albumDescription: String,
    year: i32,
    songCount: i32,
    songs: Vec<entity::song::Model>,
}
pub async fn get_album(
    Path(album_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<AlbumResponse>, (StatusCode, String)> {
    let album = services::album::get_album_by_id(&state.database, album_id).await;
    match album.ok() {
        Some(_album) => match _album.first() {
            Some(f) => {
                let album_model = f.0.to_owned();
                let songs = f.1.to_owned();
                Ok(Json(AlbumResponse {
                    id: album_model.id,
                    name: album_model.name,
                    artist: album_model.artist_name,
                    albumDescription: album_model.album_description.unwrap_or_default(),
                    artistId: album_model.artist_id.unwrap_or_default(),
                    year: album_model.year,
                    songCount: songs.len() as i32,
                    songs,
                }))
            }
            None => Err((StatusCode::ACCEPTED, "Failed to find album".to_owned())),
        },
        None => Err((StatusCode::ACCEPTED, "Failed to find album".to_owned())),
    }
}
/* #[axum_macros::debug_handler]
 */
pub async fn get_cover(
    State(state): State<AppState>,
    Path(album_id): Path<String>,
) -> Result<Response<BoxBody>, (StatusCode, String)> {
    let res: Request<Body> = Request::builder().uri("/").body(Body::empty()).unwrap();

    let album: Option<entity::album::Model> = entity::album::Entity::find_by_id(album_id)
        .one(&state.database)
        .await
        .unwrap();

    match album {
        Some(f) => {
            if f.cover.is_some() {
                // Serve image from FS
                match ServeFile::new(f.cover.unwrap()).oneshot(res).await {
                    Ok(res) => Ok(res.map(boxed)),
                    Err(err) => Err((
                        StatusCode::NOT_FOUND,
                        format!("Something went wrong: {}", err),
                    )),
                }
            } else {
                // Serve unknown album image
                let unknown_album = ASSETS.get_file("unknown_album.jpg").unwrap();
                let body = boxed(Full::from(unknown_album.contents()));
                Ok(Response::builder()
                    .header(header::CONTENT_TYPE, "image/jpg")
                    .body(body)
                    .unwrap())
            }
        }
        None => Err((StatusCode::NOT_FOUND, "Unable to find album".to_string())),
    }
}

pub async fn get_albums(
    State(state): State<AppState>,
    axum::extract::Query(params): axum::extract::Query<HashMap<String, String>>,
) -> Result<Json<Vec<entity::album::Model>>, (StatusCode, String)> {
    if params.get("size").is_some() {
        let size: u64 = params.get("size").unwrap().parse::<u64>().unwrap_or(10);
        let albums: Result<Vec<entity::album::Model>, anyhow::Error> =
            services::album::get_albums_paginate(
                &state.database,
                params
                    .get("page")
                    .unwrap_or(&String::from("0"))
                    .parse::<u64>()
                    .unwrap_or(0),
                size,
            )
            .await;
        match albums {
            Ok(_albums) => Ok(Json(_albums)),
            Err(err) => Err((
                StatusCode::ACCEPTED,
                format!("Failed to get albums {}", err),
            )),
        }
    } else {
        let albums: Result<Vec<entity::album::Model>, anyhow::Error> =
            services::album::get_all_albums(&state.database).await;
        match albums {
            Ok(_albums) => Ok(Json(_albums)),
            Err(err) => Err((
                StatusCode::ACCEPTED,
                format!("Failed to get albums {}", err),
            )),
        }
    }
}
/* pub async fn get_album_page(
    Extension(ref db): Extension<DatabaseConnection>,
    axum::extract::Query(params): axum::extract::Query<HashMap<String, String>>,
) -> Result<Json<Vec<entity::album::Model>>, (StatusCode, String)> {
    println!("{:?}", params);
    let albums = services::album::get_albums_paginate(
        db,
        0,
        params.get("size").unwrap().parse::<usize>().unwrap(),
    )
    .await;
    match albums {
        Ok(_albums) => return Ok(Json(_albums)),
        Err(err) => Err((
            StatusCode::ACCEPTED,
            format!("Failed to get albums {}", err),
        )),
    }
} */
