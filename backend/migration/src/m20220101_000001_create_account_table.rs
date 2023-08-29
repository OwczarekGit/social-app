use sea_orm_migration::prelude::*;
use crate::extension::postgres::Type;
use crate::sea_orm::{EnumIter, Iterable};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        let _ = manager.create_type(
            Type::create()
                .as_enum(AccountType::Table)
                .values([AccountType::User, AccountType::Admin])
                .to_owned()
        ).await;

        manager
            .create_table(
                Table::create()
                    .table(Account::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Account::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Account::Email).string().not_null())
                    .col(ColumnDef::new(Account::Password).string().not_null())
                    .col(ColumnDef::new(Account::Joined).timestamp().not_null())
                    .col(
                        ColumnDef::new(Account::Type)
                            .enumeration(AccountType::Table, [AccountType::User, AccountType::Admin])
                            .not_null()
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Account::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Account {
    Table,
    Id,
    Type,
    Email,
    Password,
    Joined
}

#[derive(Iden, EnumIter)]
pub enum AccountType {
    Table,
    #[iden = "User"]
    User,
    #[iden = "Admin"]
    Admin,
}
