use sea_orm::{ entity::prelude::*, ActiveValue::Set };
use serde::{Deserialize, Serialize};
use async_trait::async_trait;
use chrono::{Duration, Utc};
use rand::{distributions::Alphanumeric, Rng};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "auth_method_passes", schema_name = "public")]
pub struct Model {
  #[sea_orm(primary_key, auto_increment = false)]
  #[serde(skip_deserializing)]
  pub id: Uuid,
  #[sea_orm(unique)]
  pub user_id: Uuid,
  pub email_id: Option<Uuid>,
  pub phone_id: Option<Uuid>,
  #[serde(skip_serializing)]
  #[sea_orm(unique,nullable)]
  pub link_hash: Option<String>,
  #[sea_orm(nullable)]
  pub link_hash_expires_at: Option<ChronoDateTimeUtc>,
  pub link_used_at: Option<ChronoDateTimeUtc>,
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
  #[sea_orm(
    belongs_to = "super::phone::Entity",
    from = "Column::PhoneId",
    to = "super::phone::Column::Id",
    on_update = "Cascade",
    on_delete = "Cascade"
  )]
  Phone,
  #[sea_orm(
    belongs_to = "super::email::Entity",
    from = "Column::EmailId",
    to = "super::email::Column::Id",
    on_update = "Cascade",
    on_delete = "Cascade"
  )]
  Email,
}

impl Related<super::user::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::User.def()
  }
}

impl Related<super::email::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Email.def()
  }
}

impl Related<super::phone::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Phone.def()
  }
}

pub fn generate_expires_at() -> ChronoDateTimeUtc {
  let link_valid_mins: i64 = 10;
  return Utc::now() + Duration::minutes(link_valid_mins);
}

pub fn generate_login_link_hash() -> String {
  let link_hash_size: usize = 256;
  rand::thread_rng()
  .sample_iter(&Alphanumeric)
  .take(link_hash_size)
  .map(char::from)
  .collect()
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
  /// Create a new ActiveModel with default values. Also used by `Default::default()`.
  fn new() -> Self {
    Self {
      id: Set(Uuid::new_v4()),
      link_hash: Set(Some(generate_login_link_hash())),
      link_hash_expires_at: Set(Some(generate_expires_at())),
      created_at: Set(Utc::now()),
      updated_at: Set(Utc::now()),
      ..ActiveModelTrait::default()
    }
  }

  /// Will be triggered before insert / update
  async fn before_save<C>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr>
  where
    C: ConnectionTrait,
  {
    if !insert {
      self.updated_at = Set(Utc::now());
      // Link used means it's no longer valid so null it
      if self.link_used_at.is_set() {
        self.link_hash = Set(None);
      }
      if self.link_hash.is_set() {
        // If link hash is being set then update the expires at date
        if self.link_hash.as_ref().is_some() {
          self.link_hash = Set(Some(generate_login_link_hash()));
          self.link_hash_expires_at = Set(Some(generate_expires_at()));
        // If link hash is empty then set expiry to empty
        } else {
          self.link_hash_expires_at = Set(None);
        }
      }
    }
    Ok(self)
  }

  // Triggered after insert / update
  // async fn after_save<C>(model: Model, db: &C, insert: bool) -> Result<Model, DbErr>
  // where
  //   C: ConnectionTrait,
  // {
  //   Ok(model)
  // }

  // Will be triggered before delete
  // async fn before_delete<C>(self, db: &C) -> Result<Self, DbErr>
  // where
  //     C: ConnectionTrait,
  // {
  //     Ok(self)
  // }

  // Will be triggered after delete
  // async fn after_delete<C>(self, db: &C) -> Result<Self, DbErr>
  // where
  //     C: ConnectionTrait,
  // {
  //     Ok(self)
  // }
}