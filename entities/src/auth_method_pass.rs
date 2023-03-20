use sea_orm::{ entity::prelude::*, ActiveValue::Set };
use serde::{Deserialize, Serialize};
use async_trait::async_trait;
use chrono::{Duration, Utc};
use rand::{distributions::Alphanumeric, Rng};
use bcrypt::{
  hash_with_result as bcrypt_hash_with_result,
  verify as bcrypt_verify,
  Version as BcryptVersion,
  BcryptError, HashParts
};

#[derive(Debug, Clone, Eq, PartialEq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "password_hash_cipher")]
pub enum PassHashCipher {
  #[sea_orm(string_value = "Bcrypt")]
  Bcrypt,
}

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "auth_method_passes", schema_name = "public")]
pub struct Model {
  #[sea_orm(primary_key, auto_increment = false)]
  #[serde(skip_deserializing)]
  pub id: Uuid,
  pub user_id: Uuid,
  #[serde(skip_serializing)]
  #[sea_orm(nullable)]
  pub pass_hash: Option<String>,
  #[serde(skip_serializing)]
  pub pass_hash_cipher: PassHashCipher,
  pub pass_last_changed_at: ChronoDateTimeUtc,
  pub force_pass_change: bool,
  #[sea_orm(nullable, unique)]
  pub pass_reset_code: Option<String>, // Shorter reset code for sending via SMS
  #[sea_orm(nullable, unique)]
  pub pass_reset_str: Option<String>, // Longer reset string for including in links
  #[sea_orm(nullable)]
  pub pass_reset_code_expires_at: Option<ChronoDateTimeUtc>,
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
}

impl Related<super::user::Entity> for Entity {
  fn to() -> RelationDef {
      Relation::User.def()
  }
}

pub fn hash_pass(pass: &Option<String>, cipher: &PassHashCipher) -> Option<String> {
  if pass.is_none() {
    return None
  }
  let pass_value: String = pass.as_ref().unwrap().to_string();
  match cipher {
    PassHashCipher::Bcrypt => {
      let bcrypt_cost: u32 = 32;
      let bcrypt_version: BcryptVersion = BcryptVersion::TwoB;
      let hash_result: Result<HashParts,BcryptError> = bcrypt_hash_with_result(pass_value, bcrypt_cost);
      Some(hash_result.unwrap().format_for_version(bcrypt_version))
    }
  }
}

pub fn verify_pass_hash(pass: Option<String>, hash: Option<&String>, cipher: &PassHashCipher) -> bool {
  if pass.is_none() || hash.is_none() { 
    false
  } else {
    match cipher {
      PassHashCipher::Bcrypt => {
        bcrypt_verify(pass.unwrap(), hash.unwrap()).unwrap()
      }
    }
  }
}

pub fn gen_pass_reset_codes() -> (String, u32) {
  let mut rng = rand::thread_rng();
  (rand::thread_rng()
  .sample_iter(&Alphanumeric)
  .take(128)
  .map(char::from)
  .collect(), rng.gen_range(1..99999999))
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
      // If the pass_hash_cipher is changed but pass_hash isn't also set then clear password
      // An unusual situation but it will force the user to reset the password
      if self.pass_hash_cipher.is_set() && self.pass_hash.is_not_set() {
        self.pass_hash = Set(None);
      }
    }
    // If the password is set then hash it*
    if self.pass_hash.is_set() {
      self.pass_hash = Set(hash_pass(self.pass_hash.as_ref(),self.pass_hash_cipher.as_ref()));
    }
    // Normally these will be set together but lets check both here
    if self.pass_reset_code.is_set() || self.pass_reset_str.is_set() {
      // If either value is set and isn't None then update our reset code expiry
      if self.pass_reset_code.as_ref().is_some() || self.pass_reset_str.as_ref().is_some() {
        self.pass_reset_code_expires_at = Set(Some(Utc::now() + Duration::hours(24)));
      }
    }
    if self.pass_last_changed_at.is_set() {
      self.pass_last_changed_at = Set(Utc::now());
    }
    Ok(self)
      //     Err(DbErr::Custom(format!(
      //         "[before_save] Invalid Price, insert: {}",
      //         insert
      //     )))
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