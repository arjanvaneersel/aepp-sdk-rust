use hyper;
use serde_json;

#[derive(Debug)]
pub enum Error {
    Hyper(hyper::Error),
    Serde(serde_json::Error),
}

impl From<hyper::Error> for Error {
    fn from(e: hyper::Error) -> Self {
        return Error::Hyper(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        return Error::Serde(e)
    }
}

use super::models::*;

mod account_api;
pub use self::account_api::{ AccountApi, AccountApiClient };
mod chain_api;
pub use self::chain_api::{ ChainApi, ChainApiClient };
mod channel_api;
pub use self::channel_api::{ ChannelApi, ChannelApiClient };
mod contract_api;
pub use self::contract_api::{ ContractApi, ContractApiClient };
mod debug_api;
pub use self::debug_api::{ DebugApi, DebugApiClient };
mod external_api;
pub use self::external_api::{ ExternalApi, ExternalApiClient };
mod internal_api;
pub use self::internal_api::{ InternalApi, InternalApiClient };
mod name_service_api;
pub use self::name_service_api::{ NameServiceApi, NameServiceApiClient };
mod node_info_api;
pub use self::node_info_api::{ NodeInfoApi, NodeInfoApiClient };
mod oracle_api;
pub use self::oracle_api::{ OracleApi, OracleApiClient };
mod transaction_api;
pub use self::transaction_api::{ TransactionApi, TransactionApiClient };

pub mod configuration;
pub mod client;
