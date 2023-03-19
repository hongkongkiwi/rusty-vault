use sea_orm::{ entity::prelude::*, ActiveValue::Set };
use serde::{Deserialize, Serialize};
//use serde_email::Email;
use chrono::{Duration, Utc};
use async_trait::async_trait;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "user_emails", schema_name = "public")]
pub struct Model {
  #[sea_orm(primary_key, auto_increment = false)]
  // #[serde(skip_deserializing)]
  pub id: Uuid,
  pub user_id: Uuid,
  pub email_address: String,
  #[sea_orm(nullable)]
  pub verification_code: Option<String>,
  #[sea_orm(nullable)]
  pub verification_code_expires_at: Option<ChronoDateTimeUtc>,
  pub is_primary: bool,
  pub is_verified: bool,
  #[sea_orm(nullable)]
  pub verified_at: Option<ChronoDateTimeUtc>,
  pub created_at: ChronoDateTimeUtc,
  pub updated_at: ChronoDateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(
    belongs_to = "super::user::Entity",
    from = "Column::UserId",
    to = "super::user::Column::Id",
    on_update = "Cascade",
    on_delete = "Cascade"
  )]
  User,
}

impl Related<super::user::Entity> for Entity {
  fn to() -> RelationDef {
      Relation::User.def()
  }
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
  /// Create a new ActiveModel with default values. Also used by `Default::default()`.
  fn new() -> Self {
    Self {
      id: Set(Uuid::new_v4()),
      ..ActiveModelTrait::default()
    }
  }

  /// Will be triggered before insert / update
  async fn before_save<C>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr>
  where
    C: ConnectionTrait,
  {
    if insert {
      self.created_at = Set(chrono::Utc::now());
      self.updated_at = Set(chrono::Utc::now());
    } else {
      self.updated_at = Set(chrono::Utc::now());
    }
    // When a email becomes verified, updated the verified_at date
    if self.is_verified.is_set() {
      if *self.is_verified.as_ref() {
        self.verified_at = Set(Some(Utc::now()));
      } else {
        self.verified_at = Set(None);
      }
    }
    Ok(self)
  }
}