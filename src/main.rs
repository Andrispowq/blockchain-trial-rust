mod services;
mod api;
mod dto;

use std::sync::Arc;
use axum::Router;
use axum::routing::{get};
use tokio::sync::Mutex;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::api::accounts::{account_query, AccountServiceState};
use crate::api::ApiDoc;
use crate::services::account_service::AccountService;

const ACCOUNT: &str = "14s3KFN3AHnQ8xji3cd7BEMzF4ciipNRv3azgQwjFrf5seaW";

#[subxt::subxt(runtime_metadata_path = "metadata.scale")]
pub mod polkadot {}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let service = AccountService::new().await
        .expect("Failed to create the account service");

    let state: AccountServiceState = Arc::new(Mutex::new(service));

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
