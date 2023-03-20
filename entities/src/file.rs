use sea_orm::{ entity::prelude::*, ActiveValue::Set };
use serde::{Deserialize, Serialize};
// use url::Url;
use async_trait::async_trait;
use chrono::Utc;

// use s3::bucket::Bucket;
// use s3::creds::Credentials;
// use anyhow::Result;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "files", schema_name = "public")]
pub struct Model {
  #[sea_orm(primary_key, auto_increment = false)]
  #[serde(skip_deserializing)]
  pub id: Uuid,
  // #[sea_orm(column_type = "Text")]
  // #[serde(with = "url_serde")]
  pub s3_file_url: String,
  pub created_at: ChronoDateTimeUtc,
  pub updated_at: ChronoDateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(has_many = "super::user_profile::Entity")]
  UserProfile,
  #[sea_orm(has_many = "super::organisation_profile::Entity")]
  OrganisationProfile,
  #[sea_orm(has_many = "super::group::Entity")]
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

impl Related<super::organisation_profile::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::OrganisationProfile.def()
  }
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
      self.updated_at = Set(Utc::now());
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