use extension::postgres::Type;
use sea_orm::{EnumIter, Iterable};
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(AccountType::Table)
                    .values([
                        AccountType::User,
                        AccountType::Moderator,
                        AccountType::Admin,
                    ])
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Account::Table)
                    .if_not_exists()
                    .col(
                        pk_auto(Account::Id)
                            .big_integer()
                            .auto_increment()
                            .not_null(),
                    )
                    .col(string(Account::Email).unique_key().not_null())
                    .col(string(Account::Password).not_null())
                    .col(
                        timestamp(Account::JoinedDate)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        enumeration(Account::Type, AccountType::Table, AccountType::iter())
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Account::Table).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(AccountType::Table).to_owned())
            .await
    }
}

#[derive(Iden, EnumIter)]
pub enum AccountType {
    Table,
    #[iden = "User"]
    User,
    #[iden = "Moderator"]
    Moderator,
    #[iden = "Admin"]
    Admin,
}

#[derive(DeriveIden)]
pub enum Account {
    Table,
    Id,
    Email,
    Password,
    Type,
    JoinedDate,
}
