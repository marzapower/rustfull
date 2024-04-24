use std::fmt::{Debug, Display, Formatter};

use axum::{
    extract::{FromRequestParts, Path, State},
    http::{request::Parts, StatusCode},
    response::IntoResponse,
    Json, Router,
};
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

#[derive(Deserialize)]
pub struct GetById<T> {
    id: T,
}

// I would recommend using the derive macro #[derive(TypedPath)] but since that
// currently doesn't support generics I felt so free and inlined this here, while
// creating a pr in axum-extra to fix that - should probably be able to be replaced with
// the derive macro afterwards
impl<T: Display> TypedPath for GetById<T> {
    const PATH: &'static str = "/:id";
}

impl<T: Display> Display for GetById<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Self { id } = self;
        write!(
            f,
            "/{id}",
            id = axum_extra::__private::utf8_percent_encode(
                &id.to_string(),
                axum_extra::__private::PATH_SEGMENT,
            )
        )
    }
}

#[axum::async_trait]
impl<T, S> FromRequestParts<S> for GetById<T>
where
    S: Send + Sync,
    T: Send + Sync,
    for<'de> T: Deserialize<'de>,
{
    type Rejection = <Path<Self> as FromRequestParts<S>>::Rejection;
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        Path::from_request_parts(parts, state)
            .await
            .map(|path| path.0)
    }
}
/// till here

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

#[derive(Deserialize)]
pub struct DeleteById<T> {
    id: T,
}

// I would recommend using the derive macro #[derive(TypedPath)] but since that
// currently doesn't support generics I felt so free and inlined this here, while
// creating a pr in axum-extra to fix that - should probably be able to be replaced with
// the derive macro afterwards
impl<T: Display> TypedPath for DeleteById<T> {
    const PATH: &'static str = "/:id";
}

impl<T: Display> Display for DeleteById<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Self { id } = self;
        write!(
            f,
            "/{id}",
            id = axum_extra::__private::utf8_percent_encode(
                &id.to_string(),
                axum_extra::__private::PATH_SEGMENT,
            )
        )
    }
}

#[axum::async_trait]
impl<T, S> FromRequestParts<S> for DeleteById<T>
where
    S: Send + Sync,
    T: Send + Sync,
    for<'de> T: Deserialize<'de>,
{
    type Rejection = <Path<Self> as FromRequestParts<S>>::Rejection;
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        Path::from_request_parts(parts, state)
            .await
            .map(|path| path.0)
    }
}
/// till here

pub async fn delete_by_id<T: EntityTrait>(
    DeleteById { id: _ }: DeleteById<IdType<T>>,
    State(AppState {
        database_connection: _,
        ..
    }): State<AppState>,
) -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
}

#[derive(Deserialize)]
pub struct UpdateById<T> {
    id: T,
}

// I would recommend using the derive macro #[derive(TypedPath)] but since that
// currently doesn't support generics I felt so free and inlined this here, while
// creating a pr in axum-extra to fix that - should probably be able to be replaced with
// the derive macro afterwards
impl<T: Display> TypedPath for UpdateById<T> {
    const PATH: &'static str = "/:id";
}

impl<T: Display> Display for UpdateById<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Self { id } = self;
        write!(
            f,
            "/{id}",
            id = axum_extra::__private::utf8_percent_encode(
                &id.to_string(),
                axum_extra::__private::PATH_SEGMENT,
            )
        )
    }
}

#[axum::async_trait]
impl<T, S> FromRequestParts<S> for UpdateById<T>
where
    S: Send + Sync,
    T: Send + Sync,
    for<'de> T: Deserialize<'de>,
{
    type Rejection = <Path<Self> as FromRequestParts<S>>::Rejection;
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        Path::from_request_parts(parts, state)
            .await
            .map(|path| path.0)
    }
}
/// till here

pub async fn update_by_id<T: EntityTrait>(
    UpdateById { id: _ }: UpdateById<IdType<T>>,
    State(AppState {
        database_connection: _,
        ..
    }): State<AppState>,
) -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
}
