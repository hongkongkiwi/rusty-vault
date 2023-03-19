use sea_orm::{ entity::prelude::*, ActiveValue::Set };
use async_trait::async_trait;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "users_groups_group_auth_roles", schema_name = "public")]
pub struct Model {
  #[sea_orm(primary_key, auto_increment = false)]
  pub user_id: Uuid,
  #[sea_orm(primary_key, auto_increment = false)]
  pub group_id: Uuid,
  pub group_auth_role_id: Uuid,
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
        belongs_to = "super::group::Entity",
        from = "Column::GroupId",
        to = "super::group::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Group,
    #[sea_orm(
      belongs_to = "super::group_auth_role::Entity",
      from = "Column::GroupAuthRoleId",
      to = "super::group_auth_role::Column::Id",
      on_update = "Cascade",
      on_delete = "Cascade"
    )]
    GroupAuthRole,
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
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