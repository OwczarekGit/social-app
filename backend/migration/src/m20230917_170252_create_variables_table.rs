use sea_orm_migration::prelude::*;

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
                        ColumnDef::new(Variables::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Variables::Key).unique_key().string().not_null())
                    .col(ColumnDef::new(Variables::Value).string().not_null())
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
enum Variables {
    Table,
    Id,
    Key,
    Value,
}
