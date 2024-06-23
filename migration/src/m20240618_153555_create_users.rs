use sea_orm_migration::prelude::*;

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
                    .col(
                        ColumnDef::new(Users::Id)
                            .integer()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Users::Pid).uuid().unique_key().not_null())
                    .col(
                        ColumnDef::new(Users::Email)
                            .unique_key()
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Users::FirstName).string().not_null())
                    .col(ColumnDef::new(Users::LastName).string().not_null())
                    .col(ColumnDef::new(Users::LoginToken).string().null())
                    .col(ColumnDef::new(Users::LoginSentAt).timestamp().null())
                    .col(ColumnDef::new(Users::ResetToken).string().null())
                    .col(ColumnDef::new(Users::ResetSentAt).timestamp().null())
                    .col(
                        ColumnDef::new(Users::EmailVerificationToken)
                            .string()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Users::EmailVerificationSentAt)
                            .timestamp()
                            .null(),
                    )
                    .col(ColumnDef::new(Users::EmailVerifiedAt).timestamp().null())
                    .col(sea_query::ColumnDef::new(Users::Status).string_len(1))
                    .col(
                        ColumnDef::new(Users::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Users::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Users {
    Table,
    Id,
    Pid,
    Email,
    FirstName,
    LastName,
    LoginToken,
    LoginSentAt,
    ResetToken,
    ResetSentAt,
    EmailVerificationToken,
    EmailVerificationSentAt,
    EmailVerifiedAt,
    Status,
    CreatedAt,
    UpdatedAt,
}
