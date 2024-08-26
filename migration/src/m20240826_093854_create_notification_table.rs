use sea_orm_migration::{prelude::*, schema::*};

use crate::m20240826_090118_create_account_table::Account;

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
                        pk_auto(Notification::Id)
                            .big_integer()
                            .auto_increment()
                            .not_null(),
                    )
                    .col(string(Notification::Content).json().not_null())
                    .col(
                        string(Notification::Date)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(big_integer(Notification::AccountId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Notification::Table, Notification::AccountId)
                            .to(Account::Table, Account::Id),
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
