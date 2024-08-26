pub use sea_orm_migration::prelude::*;

mod m20240826_090118_create_account_table;
mod m20240826_093854_create_notification_table;
mod m20240826_094645_create_variables_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240826_090118_create_account_table::Migration),
            Box::new(m20240826_093854_create_notification_table::Migration),
            Box::new(m20240826_094645_create_variables_table::Migration),
        ]
    }
}
