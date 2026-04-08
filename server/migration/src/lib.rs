mod m20251225_140002_create_user_table;
mod m20251225_144622_create_otp_table;
mod m20251225_144754_update_otp_code;
mod m20251225_144927_rename_table;
mod m20251225_145435_add_fk_to_otp;
mod m20251225_145710_add_profile_picture_to_users;
mod m20251225_150002_create_notification_table;
mod m20251225_150219_add_username_to_users_table;
mod m20251225_150349_add_countries;
mod m20251227_224856_make_profile_picture_nullable;
mod m20251227_225947_make_user_name_nullable;

#[allow(unused_imports)]
pub use sea_orm_migration::prelude::*;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migration_table_name() -> sea_orm::DynIden {
        "orchard_migrations".into_iden()
    }

    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20251225_140002_create_user_table::Migration),
            Box::new(m20251225_144622_create_otp_table::Migration),
            Box::new(m20251225_144754_update_otp_code::Migration),
            Box::new(m20251225_144927_rename_table::Migration),
            Box::new(m20251225_145435_add_fk_to_otp::Migration),
            Box::new(m20251225_145710_add_profile_picture_to_users::Migration),
            Box::new(m20251225_150002_create_notification_table::Migration),
            Box::new(m20251225_150219_add_username_to_users_table::Migration),
            Box::new(m20251225_150349_add_countries::Migration),
            Box::new(m20251227_224856_make_profile_picture_nullable::Migration),
            Box::new(m20251227_225947_make_user_name_nullable::Migration),
        ]
    }
}
