use sea_orm::{ entity::prelude::*, ActiveValue::Set };
use serde::{Deserialize, Serialize};
use async_trait::async_trait;
use names::{Generator, Name};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "user_profiles", schema_name = "public")]
pub struct Model {
  #[sea_orm(primary_key, auto_increment = false)]
  // #[serde(skip_deserializing)]
  pub id: Uuid,
  pub user_id: Uuid,
  pub username: Option<String>,
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
    belongs_to = "super::user::Entity",
    from = "Column::UserId",
    to = "super::user::Column::Id"
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
    // If no username is set, generate something random
    if self.username.is_set() && self.username.as_ref().is_none() {
      let mut generator: Generator = Generator::with_naming(Name::Numbered);
      self.username = Set(generator.next());
    }
    Ok(self)
  }
}