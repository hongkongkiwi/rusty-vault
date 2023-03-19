use sea_orm::{ entity::prelude::*, ActiveValue::Set };
use serde::{Deserialize, Serialize};
use async_trait::async_trait;
use chrono::{Duration, Utc};

#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "locked_state")]
pub enum LockedState {
  #[sea_orm(string_value = "Unlocked")]
  Unlocked,
  #[sea_orm(string_value = "TemporarilyLocked")]
  TemporarilyLocked,
  #[sea_orm(string_value = "PermanentlyLocked")]
  PermanentlyLocked,
}

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "users", schema_name = "public")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    // #[serde(skip_deserializing)]
    pub id: Uuid,
    //pub auth_pass_id: Uuid,
    pub invalid_login_attempts: u16,
    pub locked_state: LockedState,
    pub locked_state_updated_at: ChronoDateTimeUtc,
    #[sea_orm(nullable)]
    pub locked_state_expires_at: Option<ChronoDateTimeUtc>,
    #[sea_orm(nullable)]
    pub last_login_at: Option<ChronoDateTimeUtc>,
    pub created_at: ChronoDateTimeUtc,
    pub updated_at: ChronoDateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(has_one = "super::user_profile::Entity")]
  UserProfile,
  #[sea_orm(has_one = "super::auth_method_pass::Entity")]
  UserAuthPass,
  #[sea_orm(has_many = "super::user_email::Entity")]
  UserEmail,
  #[sea_orm(has_many = "super::user_phone::Entity")]
  UserPhone,
}

// `Related` trait has to be implemented by hand
impl Related<super::user_email::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::UserEmail.def()
  }
}

impl Related<super::user_phone::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::UserPhone.def()
  }
}

impl Related<super::group::Entity> for Entity {
  fn to() -> RelationDef {
      super::users_groups_group_auth_roles::Relation::Group.def()
  }

  fn via() -> Option<RelationDef> {
      Some(super::users_groups_group_auth_roles::Relation::User.def().rev())
  }
}

impl Related<super::group_auth_role::Entity> for Entity {
  fn to() -> RelationDef {
      super::users_groups_group_auth_roles::Relation::GroupAuthRole.def()
  }

  fn via() -> Option<RelationDef> {
      Some(super::users_groups_group_auth_roles::Relation::User.def().rev())
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
      let locked_state = *self.locked_state.as_ref();
      // If invalid_login_attempts is changed and we are not permanently locked
      if self.invalid_login_attempts.is_set() && locked_state != LockedState::PermanentlyLocked {
        let invalid_lock_attempts = *self.invalid_login_attempts.as_ref();
        // If invalid_login_attempts is greater than 0 and we are not already temporarily locked
        if invalid_lock_attempts > 0 && locked_state != LockedState::TemporarilyLocked {  
          let max_login_attempts: u16 = 10;
          let locked_duration_mins: i64 = 60;
          // If login attempts exceeds our max then temporarily lock the account
          if invalid_lock_attempts > max_login_attempts && locked_state == LockedState::Unlocked {
            self.locked_state = Set(LockedState::TemporarilyLocked);
            self.locked_state_expires_at = Set(Some(Utc::now() + Duration::minutes(locked_duration_mins)));
            self.locked_state_updated_at = Set(Utc::now());
          }
        // If invalid_login_attempts is 0 and we are not currently unlocked then unlock
        } else if invalid_lock_attempts == 0 && locked_state != LockedState::Unlocked {
          self.locked_state = Set(LockedState::Unlocked);
          self.locked_state_expires_at = Set(None);
          self.locked_state_updated_at = Set(Utc::now());
        }
      }
      self.updated_at = Set(chrono::Utc::now());
    }
    Ok(self)
  }
}