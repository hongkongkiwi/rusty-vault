use sea_orm::{ entity::prelude::*, ActiveValue::Set };
use serde::{Deserialize, Serialize};
use async_trait::async_trait;
use chrono::{Duration, Utc};

// use s3::bucket::Bucket;
// use s3::creds::Credentials;
// use anyhow::Result;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "files", schema_name = "public")]
pub struct Model {
  #[sea_orm(primary_key, auto_increment = false)]
  #[serde(skip_deserializing)]
  pub id: Uuid,
  #[serde(with = "url_serde")]
  pub s3_file_url: Url,
  pub created_at: ChronoDateTimeUtc,
  pub updated_at: ChronoDateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(
    belongs_to = "super::user_profile::Entity",
    from = "Column::ProfileImageFileId",
    to = "super::user_profile::Column::Id",
    on_update = "Cascade",
    on_delete = "Cascade"
  )]
  UserProfile,
  #[sea_orm(
    belongs_to = "super::group::Entity",
    from = "Column::GroupImageFileId",
    to = "super::group::Column::Id",
    on_update = "Cascade",
    on_delete = "Cascade"
  )]
  Group,
}

impl Related<super::user_profile::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::UserProfile.def()
  }
}

impl Related<super::group::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Group.def()
  }
}

impl EntityAddons for ActiveModel {
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
  /// Create a new ActiveModel with default values. Also used by `Default::default()`.
  fn new() -> Self {
    Self {
      id: Set(Uuid::new_v4()),
      created_at: Set(chrono::Utc::now()),
      updated_at: Set(chrono::Utc::now()),
      ..ActiveModelTrait::default()
    }
  }

  /// Will be triggered before insert / update
  async fn before_save<C>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr>
  where
    C: ConnectionTrait,
  {
    if !insert {
      self.updated_at = Set(chrono::Utc::now());
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