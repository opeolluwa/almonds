use std::sync::Arc;

use sea_orm::{DatabaseConnection, EntityTrait};

use crate::{
    entities::{countries, prelude::Countries},
    errors::database_error::DatabaseError,
    repositories::base::Repository,
};

#[derive(Clone)]
pub struct CountryRepository {
    db_conn: Arc<DatabaseConnection>,
}

pub(crate) trait CountryRepositoryExt {
    async fn fetch_all_countries(&self) -> Result<Vec<countries::Model>, DatabaseError>;

    async fn fetch_country_by_identifier(
        &self,
        identifier: &str,
    ) -> Result<Option<countries::Model>, DatabaseError>;
}

impl Repository for CountryRepository {
    fn init(db_conn: &Arc<DatabaseConnection>) -> Self {
        Self {
            db_conn: db_conn.to_owned(),
        }
    }
}

impl CountryRepositoryExt for CountryRepository {
    async fn fetch_all_countries(&self) -> Result<Vec<countries::Model>, DatabaseError> {
        let countries: Vec<countries::Model> = Countries::find()
            .all(self.db_conn.as_ref())
            .await
            .map_err(DatabaseError::from)?;

        Ok(countries)
    }

    async fn fetch_country_by_identifier(
        &self,
        identifier: &str,
    ) -> Result<Option<countries::Model>, DatabaseError> {
        let country = Countries::find_by_id(identifier)
            .one(self.db_conn.as_ref())
            .await
            .map_err(DatabaseError::from)?;

        Ok(country)
    }
}
