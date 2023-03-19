use sea_orm::{ entity::prelude::*, ActiveValue::Set };
use serde::{Deserialize, Serialize};
use async_trait::async_trait;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "groups", schema_name = "public")]
pub struct Model {
  #[sea_orm(primary_key, auto_increment = false)]
  // #[serde(skip_deserializing)]
  pub id: Uuid,
  pub name: String,
  #[sea_orm(nullable)]
  pub description: Option<String>,
  pub icon: Option<String>,
  pub color_rgb: Option<String>,
  pub created_at: ChronoDateTimeUtc,
  pub updated_at: ChronoDateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
}

impl Related<super::user::Entity> for Entity {
  fn to() -> RelationDef {
      super::users_groups_group_auth_roles::Relation::User.def()
  }

  fn via() -> Option<RelationDef> {
      Some(super::users_groups_group_auth_roles::Relation::Group.def().rev())
  }
}

impl Related<super::group_auth_role::Entity> for Entity {
  fn to() -> RelationDef {
      super::users_groups_group_auth_roles::Relation::GroupAuthRole.def()
  }

  fn via() -> Option<RelationDef> {
      Some(super::users_groups_group_auth_roles::Relation::Group.def().rev())
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
    Ok(self)
  }
}