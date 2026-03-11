mod m20251225_140002_create_user_table;
mod m20251225_142516_create_library_table;
mod m20251225_142651_create_playlist_table;
mod m20251225_142851_create_fk_on_audio_books;
mod m20251225_143403_create_fk_on_user_playlist;
mod m20251225_143557_rename_playlist_about;
mod m20251225_144242_rename_playlist;
mod m20251225_144622_create_otp_table;
mod m20251225_144754_update_otp_code;
mod m20251225_144927_rename_table;
mod m20251225_145435_add_fk_to_otp;
mod m20251225_145710_add_profile_picture_to_users;
mod m20251225_150002_create_notification_table;
mod m20251225_150219_add_username_to_users_table;
mod m20251225_150349_add_countries;
mod m20251225_150508_add_country_identifier_to_users_table;
mod m20251225_150712_add_starred_to_audio_books;
mod m20251227_224856_make_profile_picture_nullable;
mod m20251227_225947_make_user_name_nullable;
mod m20251230_024609_support_2fa;
mod m20251230_024817_support_biometrics;
mod m20251230_025025_create_reference_email_table;
mod m20251230_025541_create_fk_with_users_and_reference_email;
mod m20260101_182310_rename_reference_email_to_backup_email;
mod m20260130_010025_create_waitlist_table;

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
            Box::new(m20251225_142516_create_library_table::Migration),
            Box::new(m20251225_142651_create_playlist_table::Migration),
            Box::new(m20251225_142851_create_fk_on_audio_books::Migration),
            Box::new(m20251225_143403_create_fk_on_user_playlist::Migration),
            Box::new(m20251225_143557_rename_playlist_about::Migration),
            Box::new(m20251225_144242_rename_playlist::Migration),
            Box::new(m20251225_144622_create_otp_table::Migration),
            Box::new(m20251225_144754_update_otp_code::Migration),
            Box::new(m20251225_144927_rename_table::Migration),
            Box::new(m20251225_145435_add_fk_to_otp::Migration),
            Box::new(m20251225_145710_add_profile_picture_to_users::Migration),
            Box::new(m20251225_150002_create_notification_table::Migration),
            Box::new(m20251225_150219_add_username_to_users_table::Migration),
            Box::new(m20251225_150349_add_countries::Migration),
            Box::new(m20251225_150508_add_country_identifier_to_users_table::Migration),
            Box::new(m20251225_150712_add_starred_to_audio_books::Migration),
            Box::new(m20251227_224856_make_profile_picture_nullable::Migration),
            Box::new(m20251227_225947_make_user_name_nullable::Migration),
            Box::new(m20251230_024609_support_2fa::Migration),
            Box::new(m20251230_024817_support_biometrics::Migration),
            Box::new(m20251230_025025_create_reference_email_table::Migration),
            Box::new(m20251230_025541_create_fk_with_users_and_reference_email::Migration),
            Box::new(m20260101_182310_rename_reference_email_to_backup_email::Migration),
            Box::new(m20260130_010025_create_waitlist_table::Migration),
        ]
    }
}
