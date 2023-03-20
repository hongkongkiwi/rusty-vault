use sea_orm_migration::{
  prelude::*,
  sea_orm::{prelude::ChronoDateTimeUtc},
  // sea_orm::{ConnectionTrait, DbBackend, Statement},
  // sea_query::extension::postgres::Type,
};

use entities::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

// #[derive(Iden)]
// enum AuthMethod {
//     #[iden = "auth_method"]
//     Type,
//     Password,
//     Google,
//     Github,
//     Discord,
// }

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    // if manager.get_database_backend() == DbBackend::Postgres {
    //   manager
    //     .create_type(Type::create()
    //       .as_enum(AuthMethod::Type)
    //       .values(vec![
    //         AuthMethod::Password,
    //         AuthMethod::Google,
    //         AuthMethod::Github,
    //         AuthMethod::Discord,
    //       ])
    //       .to_owned(),
    //     )
    //     .await?;
    // }

    // User Table
    manager
      .create_table(Table::create()
      .table(user::Entity)
      .if_not_exists()
      .col(
        ColumnDef::new(user::Column::Id)
        .uuid().not_null().primary_key())
      .col(
        ColumnDef::new(user::Column::InvalidLoginAttempts)
        .integer().default(0).not_null())
      .col(
        ColumnDef::new(user::Column::LockedState)
        .enumeration(user::LockedStateEnum, user::LockedState::iden_values())
        .default(user::LockedState::Unlocked)
        .not_null())
      .col(
        ColumnDef::new(user::Column::LockedStateUpdatedAt)
        .timestamp_with_time_zone().not_null()
        .extra("DEFAULT CURRENT_TIMESTAMP".into()))
      .col(
        ColumnDef::new(user::Column::LockedStateExpiresAt)
        .timestamp_with_time_zone()
        .default(Option::<ChronoDateTimeUtc>::None).null())
      .col(
        ColumnDef::new(user::Column::LastLoginAt)
        .timestamp_with_time_zone()
        .default(Option::<ChronoDateTimeUtc>::None).null())
      .col(
        ColumnDef::new(user::Column::CreatedAt)
        .timestamp_with_time_zone().not_null()
        .extra("DEFAULT CURRENT_TIMESTAMP".into()))
      .col(
        ColumnDef::new(user::Column::UpdatedAt)
        .timestamp_with_time_zone().not_null()
        .extra("DEFAULT CURRENT_TIMESTAMP".into()))
      .to_owned())
      .await?;

    // Auth Method Pass
    manager
      .create_table(Table::create()
      .table(auth_method_pass::Entity)
      .if_not_exists()
      .col(
        ColumnDef::new(auth_method_pass::Column::Id)
        .uuid().not_null().primary_key())
      .col(
        ColumnDef::new(auth_method_pass::Column::UserId)
        .uuid().not_null())
      .col(
        ColumnDef::new(auth_method_pass::Column::PassHash)
        .string().null())
      .col(
        ColumnDef::new(auth_method_pass::Column::PassHashCipher)
        .enumeration(auth_method_pass::PassHashCipherEnum, auth_method_pass::PassHashCipher::iden_values())
        .default(auth_method_pass::PassHashCipher::Bcrypt)
        .not_null())
      .col(
        ColumnDef::new(auth_method_pass::Column::PassLastChangedAt)
        .timestamp_with_time_zone().not_null()
        .extra("DEFAULT CURRENT_TIMESTAMP".into()))
      .col(
        ColumnDef::new(auth_method_pass::Column::ForcePassChange)
        .boolean().default(false).not_null())
      .col(
        ColumnDef::new(auth_method_pass::Column::PassResetCode)
        .string().null())
      .col(
        ColumnDef::new(auth_method_pass::Column::PassResetCodeExpiresAt)
        .timestamp_with_time_zone()
        .default(Option::<ChronoDateTimeUtc>::None).null())
      .col(
        ColumnDef::new(auth_method_pass::Column::CreatedAt)
        .timestamp_with_time_zone().not_null()
        .extra("DEFAULT CURRENT_TIMESTAMP".into()))
      .col(
        ColumnDef::new(auth_method_pass::Column::UpdatedAt)
        .timestamp_with_time_zone().not_null()
        .extra("DEFAULT CURRENT_TIMESTAMP".into()))
      .to_owned())
      .await?;

    // User Email
    manager
      .create_table(Table::create()
      .table(user_email::Entity)
      .if_not_exists()
      .col(
        ColumnDef::new(user_email::Column::Id)
        .uuid().not_null().primary_key())
      .col(
        ColumnDef::new(user_email::Column::UserId)
        .uuid().not_null())
      .col(
        ColumnDef::new(user_email::Column::EmailAddress)
        .string().not_null())
      .col(
        ColumnDef::new(user_email::Column::VerificationCode)
        .string().null())    
      .col(
        ColumnDef::new(user_email::Column::VerificationCodeExpiresAt)
        .timestamp_with_time_zone()
        .default(Option::<ChronoDateTimeUtc>::None).null())
      .col(
        ColumnDef::new(user_email::Column::IsPrimary)
        .boolean().default(false).not_null())
      .col(
        ColumnDef::new(user_email::Column::IsVerified)
        .boolean().default(false).not_null())
      .col(
        ColumnDef::new(user_email::Column::VerifiedAt)
        .timestamp_with_time_zone()
        .default(Option::<ChronoDateTimeUtc>::None).null())
      .col(
        ColumnDef::new(user_email::Column::CreatedAt)
        .timestamp_with_time_zone().not_null()
        .extra("DEFAULT CURRENT_TIMESTAMP".into()))
      .col(
        ColumnDef::new(user_email::Column::UpdatedAt)
        .timestamp_with_time_zone().not_null()
        .extra("DEFAULT CURRENT_TIMESTAMP".into()))
      .to_owned())
      .await?;

    // User Phone
    manager
      .create_table(Table::create()
      .table(user_phone::Entity)
      .if_not_exists()
      .col(
        ColumnDef::new(user_phone::Column::Id)
        .uuid().not_null().primary_key())
      .col(
        ColumnDef::new(user_phone::Column::UserId)
        .uuid().not_null())
      .col(
        ColumnDef::new(user_phone::Column::PhoneCountry)
        .integer().unsigned().not_null())
      .col(
        ColumnDef::new(user_phone::Column::PhoneNumber)
        .integer().unsigned().not_null())
      .col(
        ColumnDef::new(user_phone::Column::IsPrimary)
        .boolean().default(false).not_null())
      .col(
        ColumnDef::new(user_phone::Column::IsVerified)
        .boolean().default(false).not_null())
      .col(
        ColumnDef::new(user_email::Column::VerificationCode)
        .string().null())    
      .col(
        ColumnDef::new(user_email::Column::VerificationCodeExpiresAt)
        .timestamp_with_time_zone()
        .default(Option::<ChronoDateTimeUtc>::None).null())
      .col(
        ColumnDef::new(user_email::Column::VerifiedAt)
        .timestamp_with_time_zone()
        .default(Option::<ChronoDateTimeUtc>::None).null())
      .col(
        ColumnDef::new(user_phone::Column::VerifiedAt)
        .timestamp_with_time_zone().default(Option::<ChronoDateTimeUtc>::None))
      .col(
        ColumnDef::new(user_phone::Column::CreatedAt)
        .timestamp_with_time_zone().not_null()
        .extra("DEFAULT CURRENT_TIMESTAMP".into()))
      .col(ColumnDef::new(user_phone::Column::UpdatedAt)
      .timestamp_with_time_zone().not_null()
        .extra("DEFAULT CURRENT_TIMESTAMP".into()))
      .to_owned())
      .await?;

    // Group
    manager
      .create_table(Table::create()
      .table(group::Entity)
      .if_not_exists()
      .col(ColumnDef::new(group::Column::Id).uuid().not_null().primary_key())
      .col(ColumnDef::new(group::Column::Name).string().not_null())
      .col(ColumnDef::new(group::Column::Description).string().null())
      .col(ColumnDef::new(group::Column::CreatedAt).timestamp_with_time_zone().not_null()
      .extra("DEFAULT CURRENT_TIMESTAMP".into()))
      .col(ColumnDef::new(group::Column::UpdatedAt).timestamp_with_time_zone().not_null()
      .extra("DEFAULT CURRENT_TIMESTAMP".into()))
      .to_owned())
      .await?;
    
    // Group Auth Role
    manager
      .create_table(Table::create()
      .table(group_access_role::Entity)
      .if_not_exists()
      .col(
        ColumnDef::new(group_access_role::Column::Id)
        .uuid().not_null().primary_key())
      .col(
        ColumnDef::new(group_access_role::Column::Name)
        .string().not_null())
      .col(
        ColumnDef::new(group_access_role::Column::Description)
        .string().null())
      .col(
        ColumnDef::new(group_access_role::Column::AuthRole)
        .enumeration(group_access_role::AuthRoleEnum, group_access_role::AuthRole::iden_values())
        .not_null())
      .col(
        ColumnDef::new(group_access_role::Column::CreatedAt)
        .timestamp_with_time_zone().not_null()
        .extra("DEFAULT CURRENT_TIMESTAMP".into()))
      .col(
        ColumnDef::new(group_access_role::Column::UpdatedAt)
        .timestamp_with_time_zone().not_null()
        .extra("DEFAULT CURRENT_TIMESTAMP".into()))
      .to_owned())
      .await?;

    // Users - Groups - Group Auth Roles
    manager
      .create_table(Table::create()
      .table(users_groups_group_access_roles::Entity)
      .if_not_exists()
      .col(
        ColumnDef::new(users_groups_group_access_roles::Column::UserId)
        .uuid().not_null().primary_key())
      .col(
        ColumnDef::new(users_groups_group_access_roles::Column::GroupId)
        .uuid().not_null().primary_key())
      .col(
        ColumnDef::new(users_groups_group_access_roles::Column::GroupAuthRoleId)
        .uuid().not_null())
      .col(
        ColumnDef::new(users_groups_group_access_roles::Column::CreatedAt)
        .timestamp_with_time_zone().not_null()
      .extra("DEFAULT CURRENT_TIMESTAMP".into()))
      .col(ColumnDef::new(users_groups_group_access_roles::Column::UpdatedAt).timestamp_with_time_zone().not_null()
      .extra("DEFAULT CURRENT_TIMESTAMP".into()))
      .to_owned())
      .await?;

    Ok(())
  }

  // .col(ColumnDef::new(Chef::BakeryId).integer().not_null())
  // .foreign_key(
  //     ForeignKey::create()
  //         .name("fk-chef-bakery_id")
  //         .from(Chef::Table, Chef::BakeryId)
  //         .to(Bakery::Table, Bakery::Id),
  // )

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(Table::drop().table(user::Entity).to_owned())
      .await?;
    manager
      .drop_table(Table::drop().table(auth_method_pass::Entity).to_owned())
      .await?;
    manager
      .drop_table(Table::drop().table(user_email::Entity).to_owned())
      .await?;
    manager
      .drop_table(Table::drop().table(user_phone::Entity).to_owned())
      .await?;
    manager
      .drop_table(Table::drop().table(user_profile::Entity).to_owned())
      .await?;
    manager
      .drop_table(Table::drop().table(group::Entity).to_owned())
      .await?;
    manager
      .drop_table(Table::drop().table(group_access_role::Entity).to_owned())
      .await?;
    manager
      .drop_table(Table::drop().table(users_groups_group_access_roles::Entity).to_owned())
      .await?;
    Ok(())
  }
}

// #[derive(Iden, EnumIter)]
// pub enum GroupAuthRoleEnum {
//   Table,
//   #[iden = "Owner"]
//   Owner,
//   #[iden = "AllowOwner"]
//   AllowOwner,
//   #[iden = "AllowAdmin"]
//   AllowAdmin,
//   #[iden = "AllowReadWrite"]
//   AllowReadWrite,
//   #[iden = "AllowReadOnly"]
//   AllowReadOnly,
//   #[iden = "DeniedVisible"]
//   DeniedVisible,
//   #[iden = "DeniedBlocked"]
//   DeniedBlocked,
// }

// #[derive(Iden, EnumIter)]
// pub enum UserLockedState {
//   Table,
//   #[iden = "Unlocked"]
//   Unlocked,
//   #[iden = "TemporarilyLocked"]
//   TemporarilyLocked,
//   #[iden = "PermanentlyLocked"]
//   PermanentlyLocked,
// }

// #[derive(Iden)]
// pub enum User {
//   Table,
//   Id,
//   LockedState,
//   CreatedAt,
//   UpdatedAt,
// }

// assert_eq!(User::Table.to_string(), "user");
// assert_eq!(User::Id.to_string(), "id");
// assert_eq!(User::CreatedAt.to_string(), "created_at");
// assert_eq!(User::UpdatedAt.to_string(), "updated_at");

// #[derive(Iden)]
// pub enum Group {
//   Table,
//   Id,
//   Name,
//   CreatedAt,
//   UpdatedAt,
// }

// assert_eq!(Group::Table.to_string(), "group");
// assert_eq!(Group::Id.to_string(), "id");
// assert_eq!(Group::Name.to_string(), "name");
// assert_eq!(Group::CreatedAt.to_string(), "created_at");
// assert_eq!(Group::UpdatedAt.to_string(), "updated_at");

// #[derive(Iden)]
// pub enum UserAuthPass {
//   Table,
//   Id,
//   UserId,
//   Password,
//   CreatedAt,
//   UpdatedAt,
// }

// assert_eq!(UserAuthPass::Table.to_string(), "user_auth_pass");
// assert_eq!(UserAuthPass::Id.to_string(), "id");
// assert_eq!(UserAuthPass::UserId.to_string(), "user_id");
// assert_eq!(UserAuthPass::Password.to_string(), "password");
// assert_eq!(UserAuthPass::CreatedAt.to_string(), "created_at");
// assert_eq!(UserAuthPass::UpdatedAt.to_string(), "updated_at");

// #[derive(Iden)]
// pub enum UserEmail {
//   Table,
//   Id,
//   UserId,
//   EmailAddress,
//   IsPrimary,
//   IsVerified,
//   VerifiedAt,
//   CreatedAt,
//   UpdatedAt,
// }

// assert_eq!(UserEmail::Table.to_string(), "user_email");
// assert_eq!(UserEmail::Id.to_string(), "id");
// assert_eq!(UserEmail::UserId.to_string(), "user_id");
// assert_eq!(UserEmail::EmailAddress.to_string(), "email_address");
// assert_eq!(UserEmail::IsVerified.to_string(), "is_primary");
// assert_eq!(UserEmail::IsVerified.to_string(), "is_verified");
// assert_eq!(UserEmail::VerifiedAt.to_string(), "verified_at");
// assert_eq!(UserEmail::CreatedAt.to_string(), "created_at");
// assert_eq!(UserEmail::UpdatedAt.to_string(), "updated_at");

// #[derive(Iden)]
// pub enum UserPhone {
//   Table,
//   Id,
//   UserId,
//   PhoneCountry,
//   PhoneNumber,
//   IsPrimary,
//   IsVerified,
//   VerifiedAt,
//   CreatedAt,
//   UpdatedAt,
// }

// assert_eq!(UserPhone::Table.to_string(), "user_phone");
// assert_eq!(UserPhone::Id.to_string(), "id");
// assert_eq!(UserPhone::UserId.to_string(), "user_id");
// assert_eq!(UserPhone::PhoneCountry.to_string(), "phone_country");
// assert_eq!(UserPhone::PhoneNumber.to_string(), "phone_number");
// assert_eq!(UserPhone::IsPrimary.to_string(), "is_primary");
// assert_eq!(UserPhone::IsVerified.to_string(), "is_verified");
// assert_eq!(UserPhone::VerifiedAt.to_string(), "verified_at");
// assert_eq!(UserPhone::CreatedAt.to_string(), "created_at");
// assert_eq!(UserPhone::UpdatedAt.to_string(), "updated_at");

// #[derive(Iden)]
// pub enum UserProfile {
//   Table,
//   Id,
//   UserId,
//   Name,
//   ContactDetails,
//   IsVerified,
//   VerifiedAt,
//   CreatedAt,
//   UpdatedAt,
// }

// assert_eq!(UserProfile::Table.to_string(), "user_profile");
// assert_eq!(UserProfile::Id.to_string(), "id");
// assert_eq!(UserProfile::UserId.to_string(), "user_id");
// assert_eq!(UserProfile::Name.to_string(), "name");
// assert_eq!(UserProfile::ContactDetails.to_string(), "contact_details");
// assert_eq!(UserProfile::IsVerified.to_string(), "is_verified");
// assert_eq!(UserProfile::VerifiedAt.to_string(), "verified_at");
// assert_eq!(UserProfile::CreatedAt.to_string(), "created_at");
// assert_eq!(UserProfile::UpdatedAt.to_string(), "updated_at");

// #[derive(Iden)]
// pub enum GroupAuthRole {
//   Table,
//   Id,
//   Name,
//   Description,
//   AuthRole,
//   CreatedAt,
//   UpdatedAt,
// }

// assert_eq!(GroupAuthRole::Table.to_string(), "group_auth_role");
// assert_eq!(GroupAuthRole::Id.to_string(), "id");
// assert_eq!(GroupAuthRole::Name.to_string(), "name");
// assert_eq!(GroupAuthRole::Description.to_string(), "description");
// assert_eq!(GroupAuthRole::AuthRole.to_string(), "group_auth_role");
// assert_eq!(GroupAuthRole::CreatedAt.to_string(), "created_at");
// assert_eq!(GroupAuthRole::UpdatedAt.to_string(), "updated_at");

// #[derive(Iden)]
// pub enum UsersGroupsGroupAuthRoles {
//   Table,
//   Id,
//   UserId,
//   GroupId,
//   GroupAuthRoleId,
//   CreatedAt,
//   UpdatedAt,
// }

// assert_eq!(UsersGroupsGroupAuthRoles::Table.to_string(), "users_groups_group_role_roles");
// assert_eq!(UsersGroupsGroupAuthRoles::Id.to_string(), "id");
// assert_eq!(UsersGroupsGroupAuthRoles::UserId.to_string(), "user_id");
// assert_eq!(UsersGroupsGroupAuthRoles::GroupId.to_string(), "group_id");
// assert_eq!(UsersGroupsGroupAuthRoles::GroupAuthRoleId.to_string(), "group_auth_role_id");
// assert_eq!(UsersGroupsGroupAuthRoles::CreatedAt.to_string(), "created_at");
// assert_eq!(UsersGroupsGroupAuthRoles::UpdatedAt.to_string(), "updated_at");

// impl Migrator {
//   pub async fn run_migrations() -> Result<(), DbErr> {
//     let config = Config::new();
//     let db = create_connection(&config, false)
//         .await
//         .expect("Unable to connect to db");
//     match Migrator::up(&db, None).await {
//         Ok(_) => Ok(()),
//         Err(e) => {
//             let msg = e.to_string();
//             // This is ok, just the migrator being funky
//             if !msg.contains("been applied but its file is missing") {
//                 return Err(e);
//             }
//             Ok(())
//         }
//     }
//   }
// }