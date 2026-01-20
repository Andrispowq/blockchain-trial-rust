mod services;

use subxt::{OnlineClient, PolkadotConfig};
use subxt::utils::AccountId32;
use crate::services::account_service::AccountService;

const ACCOUNT: &str = "14s3KFN3AHnQ8xji3cd7BEMzF4ciipNRv3azgQwjFrf5seaW";

#[subxt::subxt(runtime_metadata_path = "metadata.scale")]
pub mod polkadot {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let service = AccountService::new().await?;
    let account: AccountId32 = ACCOUNT.parse()?;
    let info = service.query_account(account).await?;

    /*let api = OnlineClient::<PolkadotConfig>::from_url(ADDRESS).await?;


    let storage_query = polkadot::storage().system().account(account);

    let account_info = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?
        .expect("account entry not found");

    let free = account_info.data.free;
    let reserved = account_info.data.reserved;

    println!("free={free}, reserved={reserved}");*/

    Ok(())
}
