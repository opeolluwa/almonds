use std::sync::Arc;

use sea_orm::DatabaseConnection;

use crate::{
    adapters::country::FetchCountriesResponse,
    entities::countries,
    errors::service_error::ServiceError,
    repositories::{
        base::Repository,
        country::{CountryRepository, CountryRepositoryExt},
    },
};

#[derive(Clone)]
pub struct CountryService {
    country_repository: CountryRepository,
}

impl CountryService {
    pub fn init(db_conn: &Arc<DatabaseConnection>) -> Self {
        Self {
            country_repository: CountryRepository::init(db_conn),
        }
    }
}

pub(crate) trait CountryServiceExt {
    async fn get_all_countries(&self) -> Result<FetchCountriesResponse, ServiceError>;

    async fn get_country_by_identifier(
        &self,
        identifier: &str,
    ) -> Result<Option<countries::Model>, ServiceError>;

    async fn get_countries_by_currency_code(
        &self,
        currency_code: &str,
    ) -> Result<FetchCountriesResponse, ServiceError>;
}

impl CountryServiceExt for CountryService {
    async fn get_all_countries(&self) -> Result<FetchCountriesResponse, ServiceError> {
        let records = self
            .country_repository
            .fetch_all_countries()
            .await
            .map_err(ServiceError::from)?;

        Ok(FetchCountriesResponse { records })
    }

    async fn get_country_by_identifier(
        &self,
        identifier: &str,
    ) -> Result<Option<countries::Model>, ServiceError> {
        let country = self
            .country_repository
            .fetch_country_by_identifier(identifier)
            .await
            .map_err(ServiceError::from)?;

        Ok(country)
    }

    async fn get_countries_by_currency_code(
        &self,
        currency_code: &str,
    ) -> Result<FetchCountriesResponse, ServiceError> {
        let records = self
            .country_repository
            .fetch_countries_by_currency_code(currency_code)
            .await
            .map_err(ServiceError::from)?;

        Ok(FetchCountriesResponse { records })
    }
}
