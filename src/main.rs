mod api;
mod dto;
mod services;

use crate::api::ApiDoc;
use crate::api::accounts::{AccountRateLimiter, AccountServiceState, AccountsState, account_query};
use crate::services::account_service::AccountService;
use axum::Router;
use axum::routing::get;
use governor::Quota;
use std::num::NonZeroU32;
use std::sync::Arc;
use tokio::sync::Mutex;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

const DEFAULT_ACCOUNT_RATE_LIMIT_PER_MINUTE: u32 = 3;
const ACCOUNT_RATE_LIMIT_ENV: &str = "ACCOUNT_RATE_LIMIT_PER_MINUTE";

#[subxt::subxt(runtime_metadata_path = "metadata.scale")]
pub mod polkadot {}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let service = AccountService::new()
        .await
        .expect("Failed to create the account service");

    let service_state: AccountServiceState = Arc::new(Mutex::new(service));

    let account_rate_limit = account_rate_limit_per_minute();
    tracing::info!("rate limiting /accounts endpoint to {account_rate_limit} requests per minute");
    let quota = Quota::per_minute(
        NonZeroU32::new(account_rate_limit).expect("account rate limit must be greater than zero"),
    );
    let limiter = Arc::new(AccountRateLimiter::direct(quota));
    let state = AccountsState::new(service_state, limiter);

    let app = Router::new()
        .route("/accounts/{address}", get(account_query))
        .merge(SwaggerUi::new("/docs").url("/api-doc/openapi.json", ApiDoc::openapi()))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4500")
        .await
        .expect("failed to bind listener");
    tracing::info!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.expect("server failed");
}

fn account_rate_limit_per_minute() -> u32 {
    std::env::var(ACCOUNT_RATE_LIMIT_ENV)
        .ok()
        .and_then(|value| value.parse::<u32>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(DEFAULT_ACCOUNT_RATE_LIMIT_PER_MINUTE)
}
