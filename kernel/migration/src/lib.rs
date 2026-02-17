pub use sea_orm_migration::prelude::*;

mod m20260217_143820_create_snippet_table;
mod m20260217_150510_create_sync_queue_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260217_143820_create_snippet_table::Migration),
            Box::new(m20260217_150510_create_sync_queue_table::Migration),
        ]
    }
}
