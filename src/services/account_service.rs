use subxt::{OnlineClient, PolkadotConfig};
use subxt::utils::AccountId32;
use crate::polkadot;
use crate::polkadot::system::storage::types::account::Account;

pub struct AccountService {
    api: OnlineClient<PolkadotConfig>,
}

const ADDRESS: &str = "wss://testnet-gw1.mosaicchain.io/testnet-blockchain-1/chain";

impl AccountService {
    pub async fn new() -> Result<AccountService, Box<dyn std::error::Error>> {
        let conf = OnlineClient::<PolkadotConfig>::from_url(ADDRESS).await?;
        Ok(AccountService {
            api: conf
        })
    }

    pub async fn query_account(&self, id: AccountId32) -> Result<Account, Box<dyn std::error::Error>> {
        let storage_query = polkadot::storage().system().account(id);

        let account = self.api
            .storage()
            .at_latest()
            .await?
            .fetch(&storage_query)
            .await?;

        match account {
            Some(account) => Ok(account),
            None => Err("account not found".into()),
        }
    }
}