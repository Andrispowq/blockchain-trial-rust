use crate::dto::dto::DtoConvertible;
use crate::polkadot::system::storage::types::account::Account;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AccountDto {
    nonce: u32,
    consumers: u32,
    providers: u32,
    sufficients: u32,
    data: AccountDtoData,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AccountDtoData {
    free: u128,
    reserved: u128,
    frozen: u128,
    extra: u128,
}

impl DtoConvertible<AccountDto> for Account {
    fn to_dto(&self) -> AccountDto {
        AccountDto {
            nonce: self.nonce,
            consumers: self.consumers,
            providers: self.providers,
            sufficients: self.sufficients,
            data: AccountDtoData {
                free: self.data.free,
                reserved: self.data.reserved,
                frozen: self.data.frozen,
                extra: self.data.flags.0,
            },
        }
    }
}
