use sea_orm::{ entity::prelude::*, ActiveValue::Set };
use serde::{Deserialize, Serialize};
use async_trait::async_trait;
use chrono::Utc;
// use rand::{distributions::Alphanumeric, Rng};

#[derive(Debug, Clone, Eq, PartialEq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "pki_key_algos")]
pub enum KeyAlgos {
  #[sea_orm(string_value = "RSA")]
  RSA,
}

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "pki_key", schema_name = "public")]
pub struct Model {
  #[sea_orm(primary_key, auto_increment = false)]
  #[serde(skip_deserializing)]
  pub id: Uuid,
  pub user_id: Option<Uuid>,
  pub organisation_id: Option<Uuid>,
  pub group_id: Option<Uuid>,
  #[serde(skip_serializing)]
  #[sea_orm(unique,nullable)]
  pub private_key: Option<String>,
  pub public_key: Option<String>,
  pub aws_kms_url: Option<String>,
  pub algo: KeyAlgos,
  pub created_at: ChronoDateTimeUtc,
  pub updated_at: ChronoDateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(
    belongs_to = "super::organisation::Entity",
    from = "Column::OrganisationId",
    to = "super::organisation::Column::Id",
    on_update = "Cascade",
    on_delete = "Cascade"
  )]
  Organisation,
  #[sea_orm(
    belongs_to = "super::user::Entity",
    from = "Column::UserId",
    to = "super::user::Column::Id",
    on_update = "Cascade",
    on_delete = "Cascade"
  )]
  User,
  #[sea_orm(
    belongs_to = "super::user::Entity",
    from = "Column::GroupId",
    to = "super::user::Column::Id",
    on_update = "Cascade",
    on_delete = "Cascade"
  )]
  Group,
}

impl Related<super::organisation::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Organisation.def()
  }
}

impl Related<super::user::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::User.def()
  }
}

impl Related<super::group::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Group.def()
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
    if self.user_id.as_ref().is_none() 
      && self.organisation_id.as_ref().is_none() 
      && self.group_id.as_ref().is_none() {
      return Err(DbErr::Custom(format!(
        "[before_save] All of user_id, organisation_id and group_id cannot be blank, insert: {}",
        insert
      )));
    }
    if !insert {
      self.updated_at = Set(Utc::now());
    }
    Ok(self)
  }
}