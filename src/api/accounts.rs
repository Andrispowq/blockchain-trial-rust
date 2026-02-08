use std::sync::Arc;

use crate::dto::account_dto::AccountDto;
use crate::dto::dto::DtoConvertible;
use crate::services::account_service::AccountService;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use governor::{RateLimiter, clock::DefaultClock, state::InMemoryState, state::direct::NotKeyed};
use subxt::utils::AccountId32;
use tokio::sync::Mutex;

type ErrorResponse = (StatusCode, String);
pub type AccountServiceState = Arc<Mutex<AccountService>>;
pub type AccountRateLimiter = RateLimiter<NotKeyed, InMemoryState, DefaultClock>;

#[derive(Clone)]
pub struct AccountsState {
    pub service: AccountServiceState,
    pub rate_limiter: Arc<AccountRateLimiter>,
}

impl AccountsState {
    pub fn new(service: AccountServiceState, rate_limiter: Arc<AccountRateLimiter>) -> Self {
        Self {
            service,
            rate_limiter,
        }
    }
}

#[utoipa::path(
    get,
    path = "/accounts/{address}",
    params(
        ("address" = String, Path, description = "The address of the account to query"),
    ),
    responses(
        (status = 200, description = "The data returned from the account", body = AccountDto),
        (status = 400, description = "Account address provided is invalid"),
        (status = 404, description = "Account not found"),
        (status = 429, description = "Too many requests"),
        (status = 500, description = "Internal error"),
    ),
    tag = "accounts"
)]
pub async fn account_query(
    State(state): State<AccountsState>,
    Path(address): Path<String>,
) -> Result<(StatusCode, Json<AccountDto>), ErrorResponse> {
    let AccountsState {
        service,
        rate_limiter,
    } = state;

    if rate_limiter.check().is_err() {
        return Err((
            StatusCode::TOO_MANY_REQUESTS,
            String::from("Too many requests; wait before retrying"),
        ));
    }

    let service = service.lock().await;
    let addr = address.parse::<AccountId32>();
    let Ok(addr) = addr else {
        return Err((
            StatusCode::BAD_REQUEST,
            String::from("You provided an invalid account address"),
        ));
    };

    let result = service.query_account(addr).await;
    match result {
        Ok(account) => Ok((StatusCode::OK, Json::from(account.to_dto()))),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("Internal error"),
        )),
    }
}
