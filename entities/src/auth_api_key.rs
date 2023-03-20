use sea_orm::{ entity::prelude::*, ActiveValue::Set, DbErr };
use serde::{Deserialize, Serialize};
use chrono::Utc;
use async_trait::async_trait;
use rand::{distributions::Alphanumeric, Rng};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "api_keys", schema_name = "public")]
pub struct Model {
  #[sea_orm(primary_key, auto_increment = false)]
  #[serde(skip_deserializing)]
  pub id: Uuid,
  pub user_id: Option<Uuid>,
  pub organisation_id: Option<Uuid>,
  pub api_access_key: String,
  pub api_secret_key: String,
  pub key_issued_at: ChronoDateTimeUtc,
  pub expires_on: Option<ChronoDateTimeUtc>,
  pub ip_address_last_used: Option<String>, 
  pub key_last_used_at: Option<ChronoDateTimeUtc>,
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
    belongs_to = "super::organisation::Entity",
    from = "Column::OrganisationId",
    to = "super::organisation::Column::Id",
    on_update = "Cascade",
    on_delete = "Cascade"
  )]
  Organisation,
}

impl Related<super::user::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::User.def()
  }
}

impl Related<super::organisation::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Organisation.def()
  }
}

pub fn generante_api_key() -> (String, String) {
  let api_access_key_size: usize = 64;
  let api_secret_key_size: usize = 64;
  (rand::thread_rng()
  .sample_iter(&Alphanumeric)
  .take(api_access_key_size)
  .map(char::from)
  .collect(),
  rand::thread_rng()
  .sample_iter(&Alphanumeric)
  .take(api_secret_key_size)
  .map(char::from)
  .collect())
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
  /// Create a new ActiveModel with default values. Also used by `Default::default()`.
  fn new() -> Self {
    let (api_access_key, api_secret_key) = generante_api_key();
    Self {
      id: Set(Uuid::new_v4()),
      api_access_key: Set(api_access_key),
      api_secret_key: Set(api_secret_key),
      key_issued_at: Set(Utc::now()),
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
    if self.user_id.as_ref().is_none() && self.organisation_id.as_ref().is_none() {
      return Err(DbErr::Custom(format!(
        "[before_save] Both user_id and organisation_id cannot be blank, insert: {}",
        insert
      )));
    }
    if !insert {
      self.updated_at = Set(Utc::now());
      if self.ip_address_last_used.is_set() {
        self.key_last_used_at = Set(Some(Utc::now()));
      }
    }
    Ok(self)
  }
}