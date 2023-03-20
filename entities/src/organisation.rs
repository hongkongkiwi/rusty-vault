use sea_orm::{entity::prelude::*, ActiveValue::Set, ConnectionTrait};
use serde::{Deserialize, Serialize};
use async_trait::async_trait;
use chrono::Utc;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "organisations", schema_name = "public")]
pub struct Model {
  #[sea_orm(primary_key, auto_increment = false)]
  #[serde(skip_deserializing)]
  pub id: Uuid,
  pub name: String,
  pub pki_key_id: Uuid,
  pub created_at: ChronoDateTimeUtc,
  pub updated_at: ChronoDateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(has_one = "super::organisation_profile::Entity")]
  OrganisationProfile,
  #[sea_orm(has_many = "super::group::Entity")]
  Group,
  #[sea_orm(has_many = "super::api_key::Entity")]
  ApiKey,
  #[sea_orm(has_many = "super::pki_key::Entity")]
  PkiKey,
}

impl Related<super::organisation_profile::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::OrganisationProfile.def()
  }
}

impl Related<super::group::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Group.def()
  }
}

impl Related<super::api_key::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::ApiKey.def()
  }
}

impl Related<super::pki_key::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::PkiKey.def()
  }
}

impl Related<super::user::Entity> for Entity {
  fn to() -> RelationDef {
    super::users_organisations_organisations_access_roles::Relation::User.def()
  }
  fn via() -> Option<RelationDef> {
    Some(super::users_organisations_organisations_access_roles::Relation::Organisation.def().rev())
  }
}

impl Related<super::organisation_access_role::Entity> for Entity {
  fn to() -> RelationDef {
    super::users_organisations_organisations_access_roles::Relation::OrganisationAccessRole.def()
  }
  fn via() -> Option<RelationDef> {
    Some(super::users_organisations_organisations_access_roles::Relation::Organisation.def().rev())
  }
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
  /// Create a new ActiveModel with default values. Also used by `Default::default()`.
  fn new() -> Self {
    Self {
      id: Set(Uuid::new_v4()),
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
    }
    Ok(self)
  }
}