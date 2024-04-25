use sea_orm::DatabaseConnection;

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct AppState {
    pub database_connection: DatabaseConnection,
}
