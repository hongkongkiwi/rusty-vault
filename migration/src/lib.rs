pub use sea_orm_migration::prelude::*;
// use entities::models::create_connection;

mod m20230315_143439_create_tables;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
  fn migrations() -> Vec<Box<dyn MigrationTrait>> {
    vec![
        Box::new(m20230315_143439_create_tables::Migration),
    ]
  }
}
