use sea_orm::{ entity::prelude::*, ActiveValue::Set };
use serde::{Deserialize, Serialize};
use async_trait::async_trait;
use chrono::Utc;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "user_phones", schema_name = "public")]
pub struct Model {
  #[sea_orm(primary_key, auto_increment = false)]
  #[serde(skip_deserializing)]
  pub id: Uuid,
  pub user_id: Option<Uuid>,
  pub organisation_profile_id: Option<Uuid>,
  pub needs_verification: bool,
  pub phone_country: u32,
  pub phone_number: u32,
  pub is_primary: bool,
  pub is_verified: bool,
  #[sea_orm(nullable, unique)]
  pub verification_code: Option<String>,
  #[sea_orm(nullable)]
  pub verification_code_expires_at: Option<ChronoDateTimeUtc>,
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
  #[sea_orm(
    belongs_to = "super::organisation_profile::Entity",
    from = "Column::OrganisationProfileId",
    to = "super::organisation_profile::Column::Id",
    on_update = "Cascade",
    on_delete = "Cascade"
  )]
  OrganisationProfile,
  #[sea_orm(has_one = "super::auth_method_magiclink::Entity")]
  AuthMethodMagicLink,
}

impl Related<super::user::Entity> for Entity {
  fn to() -> RelationDef {
      Relation::User.def()
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
      self.updated_at = Set(chrono::Utc::now());
    }
    // When a phone becomes verified, updated the verified_at date
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