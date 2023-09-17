pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_account_table;
mod m20230829_174346_create_notification_table;
mod m20230917_170252_create_variables_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_account_table::Migration),
            Box::new(m20230829_174346_create_notification_table::Migration),
            Box::new(m20230917_170252_create_variables_table::Migration),
        ]
    }
}
