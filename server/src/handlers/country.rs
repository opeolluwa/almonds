use axum::extract::{Path, State};

use crate::{
    adapters::country::FetchCountriesResponse,
    entities::countries,
    errors::service_error::ServiceError,
    response::ApiResponse,
    services::country_service::{CountryService, CountryServiceExt},
};

pub async fn fetch_all_countries(
    State(country_service): State<CountryService>,
) -> Result<ApiResponse<FetchCountriesResponse>, ServiceError> {
    let countries = country_service.get_all_countries().await?;

    Ok(ApiResponse::builder()
        .message("Countries fetched successfully")
        .data(countries)
        .build())
}

pub async fn fetch_country_by_identifier(
    State(country_service): State<CountryService>,
    Path(identifier): Path<String>,
) -> Result<ApiResponse<Option<countries::Model>>, ServiceError> {
    let country = country_service
        .get_country_by_identifier(&identifier)
        .await?;

    Ok(ApiResponse::builder()
        .message("Country fetched successfully")
        .data(country)
        .build())
}

pub async fn fetch_countries_by_currency_code(
    State(country_service): State<CountryService>,
    Path(currency_code): Path<String>,
) -> Result<ApiResponse<FetchCountriesResponse>, ServiceError> {
    let countries = country_service
        .get_countries_by_currency_code(&currency_code)
        .await?;

    Ok(ApiResponse::builder()
        .message("Countries fetched successfully")
        .data(countries)
        .build())
}
