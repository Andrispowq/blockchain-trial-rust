use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode
};
use subxt::utils::AccountId32;
use tokio::sync::Mutex;
use crate::services::account_service::AccountService;
use crate::dto::account_dto::AccountDto;
use crate::dto::dto::DtoConvertible;

type ErrorResponse = (StatusCode, String);
pub type AccountServiceState = Arc<Mutex<AccountService>>;

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
        (status = 500, description = "Internal error"),
    ),
    tag = "accounts"
)]
pub async fn account_query(
    State(state): State<AccountServiceState>,
    Path(address): Path<String>,
) -> Result<(StatusCode, Json<AccountDto>), ErrorResponse> {
    let service = state.lock().await;
    let addr = address.parse::<AccountId32>();
    let Ok(addr) = addr else {
        return Err((StatusCode::BAD_REQUEST, String::from("You provided an invalid account address")));
    };

    let result = service.query_account(addr).await;
    match result {
        Ok(account) => Ok((StatusCode::OK, Json::from(account.to_dto()))),
        Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, String::from("Internal error"))),
    }
}