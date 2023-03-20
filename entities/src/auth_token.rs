use sea_orm::entity::prelude::*;
use chrono::{Duration, Utc};
use async_trait::async_trait;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "auth_tokens")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub token: String,
  // pub user_id: Uuid,
  pub refresh: Option<String>,
  pub token_type: String,
  pub owner_id: i64,
  pub expire: ChronoDateTimeUtc,
  pub client_id: Option<i64>,
  pub scope: String,
  pub created_at: ChronoDateTimeUtc,
  pub updated_at: ChronoDateTimeUtc,
}

// #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
// pub enum Relation {
//   #[sea_orm(
//     belongs_to = "super::user::Entity",
//     from = "Column::UserId",
//     to = "super::user::Column::Id"
//     on_update = "Cascade",
//     on_delete = "Cascade"
//   )]
//   User,
// }

// impl Related<super::user::Entity> for Entity {
//   fn to() -> RelationDef {
//     Relation::User.def()
//   }
// }

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
  /// Create a new ActiveModel with default values. Also used by `Default::default()`.
  fn new() -> Self {
    Self {
      created_at: Set(Utc::now()),
      updated_at: Set(Utc::now()),
      ..ActiveModelTrait::default()
    }
  }

  async fn before_save<C>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr>
  where
    C: ConnectionTrait,
  {
    if !insert {
      self.updated_at = Set(Utc::now());
    }
    Ok(self)
  }
}