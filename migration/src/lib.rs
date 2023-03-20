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

// impl Migrator {
//   pub async fn run_migrations() -> Result<(), DbErr> {
//       let config = Config::new();

//       let db = create_connection(&config, false)
//           .await
//           .expect("Unable to connect to db");

//       match Migrator::up(&db, None).await {
//           Ok(_) => Ok(()),
//           Err(e) => {
//               let msg = e.to_string();
//               // This is ok, just the migrator being funky
//               if !msg.contains("been applied but its file is missing") {
//                   return Err(e);
//               }

//               Ok(())
//           }
//       }
//   }
// }
