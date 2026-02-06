use utoipa::OpenApi;

pub mod accounts;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::api::accounts::account_query
    ),
    components(
        schemas()
    ),
    tags(
        (name = "accounts", description = "Account query endpoints")
    )
)]
pub struct ApiDoc;