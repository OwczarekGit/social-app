use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Variables::Table)
                    .if_not_exists()
                    .col(
                        pk_auto(Variables::Id)
                            .big_integer()
                            .auto_increment()
                            .not_null(),
                    )
                    .col(string(Variables::Key).unique_key().not_null())
                    .col(string(Variables::Value).not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Variables::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Variables {
    Table,
    Id,
    Key,
    Value,
}
