use sea_orm::{ entity::prelude::*, ActiveValue::Set };
use serde::{Deserialize, Serialize};
use async_trait::async_trait;
use chrono::Utc;

#[derive(Debug, Clone, Eq, PartialEq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "group_role_permissions")]
pub enum GroupRolePermissions {
  #[sea_orm(string_value = "AllowOwner")]
  AllowOwner,
  #[sea_orm(string_value = "AllowAdmin")]
  AllowAdmin,
  #[sea_orm(string_value = "AllowReadWrite")]
  AllowReadWrite,
  #[sea_orm(string_value = "AllowReadOnly")]
  AllowReadOnly,
  #[sea_orm(string_value = "Denied")]
  Denied,
  #[sea_orm(string_value = "DeniedBlocked")]
  DeniedBlocked,
}

#[derive(Debug, Clone, Eq, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "group_access_roles", schema_name = "public")]
pub struct Model {
  #[sea_orm(primary_key, auto_increment = false)]
  #[serde(skip_deserializing)]
  pub id: Uuid,
  pub name: String,
  #[sea_orm(nullable)]
  pub description: Option<String>,
  pub group_role_permissions: GroupRolePermissions,
  pub created_at: ChronoDateTimeUtc,
  pub updated_at: ChronoDateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
}

impl Related<super::user::Entity> for Entity {
  fn to() -> RelationDef {
    super::users_groups_group_access_roles::Relation::User.def()
  }
  fn via() -> Option<RelationDef> {
    Some(super::users_groups_group_access_roles::Relation::GroupAccessRole.def().rev())
  }
}

impl Related<super::group::Entity> for Entity {
  fn to() -> RelationDef {
    super::users_groups_group_access_roles::Relation::Group.def()
  }
  fn via() -> Option<RelationDef> {
    Some(super::users_groups_group_access_roles::Relation::GroupAccessRole.def().rev())
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
}