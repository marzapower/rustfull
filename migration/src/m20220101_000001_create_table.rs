// Remember to import `sea_orm_migration::schema::*`
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveIden)]
enum Users {
    Table, // this is a special case; will be mapped to `post`
    Id,
    Pid,
    Email,
    Password,
    ApiKey,
    Name,
    ResetToken,
    ResetSentAt,
    EmailVerificationToken,
    EmailVerificationSentAt,
    EmailVerifiedAt,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(pk_auto(Users::Id)) // Primary key with auto-increment
                    .col(uuid(Users::Pid)) // UUID column
                    .col(string_uniq(Users::Email)) // String column with unique and not null constraint
                    .col(string(Users::Password)) // String column
                    .col(string(Users::ApiKey).unique_key())
                    .col(string(Users::Name))
                    .col(string_null(Users::ResetToken)) // Nullable string column
                    .col(timestamp_null(Users::ResetSentAt)) // Nullable timestamp column
                    .col(string_null(Users::EmailVerificationToken))
                    .col(timestamp_null(Users::EmailVerificationSentAt))
                    .col(timestamp_null(Users::EmailVerifiedAt))
                    .to_owned(),
            )
            .await
    }
}
