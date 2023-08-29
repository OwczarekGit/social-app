use sea_orm_migration::prelude::*;
use crate::m20220101_000001_create_account_table::Account;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Notification::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Notification::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Notification::Date).timestamp().not_null())
                    .col(ColumnDef::new(Notification::Content).json().not_null())
                    .col(ColumnDef::new(Notification::AccountId).big_integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Notification::Table, Notification::AccountId)
                            .to(Account::Table, Account::Id)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Notification::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Notification {
    Table,
    Id,
    AccountId,
    Content,
    Date,
}
