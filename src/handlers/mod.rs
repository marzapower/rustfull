use std::fmt::{Debug, Display};

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json, Router};
use axum_extra::routing::{RouterExt, TypedPath};
use sea_orm::{EntityTrait, PrimaryKeyTrait};
use serde::Deserialize;

use crate::state::AppState;

#[allow(type_alias_bounds)]
pub type IdType<T>
where
    T: EntityTrait,
= <T::PrimaryKey as PrimaryKeyTrait>::ValueType;

// this is fine for quick and dirty debugging but please never use
// something like this in production since it's way to easy to accidentally
// leak unwanted data
pub fn sea_orm_entity_router<T>() -> Router<AppState>
where
    T: EntityTrait,
    IdType<T>: Debug + Display + Send + Sync,
    for<'a> IdType<T>: Deserialize<'a>,
{
    Router::new()
        .typed_get(get::<T>)
        .typed_get(get_by_id::<T>)
        .typed_post(create::<T>)
        .typed_patch(update_by_id::<T>)
        .typed_delete(delete_by_id::<T>)
}

#[derive(TypedPath, Deserialize)]
#[typed_path("/")]
pub struct Get;

pub async fn get<T: EntityTrait>(
    _: Get,
    State(AppState {
        database_connection,
        ..
    }): State<AppState>,
) -> impl IntoResponse {
    Json(
        T::find()
            .into_json()
            .all(&database_connection)
            .await
            .unwrap(),
    )
}

#[derive(TypedPath, Deserialize)]
#[typed_path("/:id")]
pub struct GetById<T> {
    id: T,
}

pub async fn get_by_id<T: EntityTrait>(
    GetById { id }: GetById<IdType<T>>,
    State(AppState {
        database_connection,
        ..
    }): State<AppState>,
) -> impl IntoResponse {
    Json(
        T::find_by_id(id)
            .into_json()
            .one(&database_connection)
            .await
            .unwrap(),
    )
}

#[derive(TypedPath, Debug)]
#[typed_path("/")]
pub struct Create;

pub async fn create<T: EntityTrait>(
    _: Create,
    State(AppState {
        database_connection: _,
        ..
    }): State<AppState>,
) -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
}

#[derive(TypedPath, Deserialize)]
#[typed_path("/:id")]
pub struct DeleteById<T> {
    id: T,
}

pub async fn delete_by_id<T: EntityTrait>(
    DeleteById { id: _ }: DeleteById<IdType<T>>,
    State(AppState {
        database_connection: _,
        ..
    }): State<AppState>,
) -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
}

#[derive(TypedPath, Deserialize)]
#[typed_path("/:id")]
pub struct UpdateById<T> {
    id: T,
}

pub async fn update_by_id<T: EntityTrait>(
    UpdateById { id: _ }: UpdateById<IdType<T>>,
    State(AppState {
        database_connection: _,
        ..
    }): State<AppState>,
) -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
}
