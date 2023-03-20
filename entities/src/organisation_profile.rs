use sea_orm::{ entity::prelude::*, ActiveValue::Set };
use serde::{Deserialize, Serialize};
use async_trait::async_trait;
use chrono::Utc;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "organisation_profiles", schema_name = "public")]
pub struct Model {
  #[sea_orm(primary_key, auto_increment = false)]
  #[serde(skip_deserializing)]
  pub id: Uuid,
  #[sea_orm(unique)]
  pub organisation_id: Uuid,
  #[sea_orm(nullable)]
  pub organisation_image_file_id: Option<Uuid>,
  // #[sea_orm(column_type = "Json")]
  pub name: String,
  pub contact_details: Json,
  #[sea_orm(nullable)]
  pub notes: Option<String>,
  pub created_at: ChronoDateTimeUtc,
  pub updated_at: ChronoDateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(
    belongs_to = "super::organisation::Entity",
    from = "Column::OrganisationId",
    to = "super::organisation::Column::Id"
    on_update = "Cascade",
    on_delete = "Cascade"
  )]
  Organisation,
  #[sea_orm(has_many = "super::file::Entity")]
  File,
  #[sea_orm(has_many = "super::phone::Entity")]
  Phone,
}

impl Related<super::organisation::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Organisation.def()
  }
}

impl Related<super::phone::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Phone.def()
  }
}

impl Related<super::file::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::File.def()
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
      self.updated_at = Set(chrono::Utc::now());
    }
    Ok(self)
  }
}