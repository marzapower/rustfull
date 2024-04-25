pub mod handlers;
pub mod state;

use tokio::net::TcpListener;

use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};

use axum::Router;

use entity::prelude::*;

use crate::handlers::sea_orm_entity_router;
use crate::state::AppState;

#[tokio::main(worker_threads = 5)]
async fn main() {
    let _res = dotenvy::dotenv();

    let database_url = std::env::var("DATABASE_URL").unwrap();
    let database_connection: DatabaseConnection = Database::connect(&database_url).await.unwrap();

    Migrator::up(&database_connection, None).await.unwrap();

    let state = AppState {
        database_connection,
    };

    let listener = TcpListener::bind("0.0.0.0:7878").await.unwrap();

    let app = Router::new()
        .nest("/user", sea_orm_entity_router::<Users>())
        .nest("/post", sea_orm_entity_router::<Post>())
        .with_state(state);

    println!("We are ready to go, again!");

    axum::serve(listener, app).await.unwrap();
}
