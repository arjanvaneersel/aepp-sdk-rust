use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient<C: hyper::client::Connect> {
  configuration: Rc<Configuration<C>>,
  account_api: Box<::apis::AccountApi>,
  chain_api: Box<::apis::ChainApi>,
  channel_api: Box<::apis::ChannelApi>,
  contract_api: Box<::apis::ContractApi>,
  debug_api: Box<::apis::DebugApi>,
  external_api: Box<::apis::ExternalApi>,
  internal_api: Box<::apis::InternalApi>,
  name_service_api: Box<::apis::NameServiceApi>,
  node_info_api: Box<::apis::NodeInfoApi>,
  oracle_api: Box<::apis::OracleApi>,
  transaction_api: Box<::apis::TransactionApi>,
}

impl<C: hyper::client::Connect> APIClient<C> {
  pub fn new(configuration: Configuration<C>) -> APIClient<C> {
    let rc = Rc::new(configuration);

    APIClient {
      configuration: rc.clone(),
      account_api: Box::new(::apis::AccountApiClient::new(rc.clone())),
      chain_api: Box::new(::apis::ChainApiClient::new(rc.clone())),
      channel_api: Box::new(::apis::ChannelApiClient::new(rc.clone())),
      contract_api: Box::new(::apis::ContractApiClient::new(rc.clone())),
      debug_api: Box::new(::apis::DebugApiClient::new(rc.clone())),
      external_api: Box::new(::apis::ExternalApiClient::new(rc.clone())),
      internal_api: Box::new(::apis::InternalApiClient::new(rc.clone())),
      name_service_api: Box::new(::apis::NameServiceApiClient::new(rc.clone())),
      node_info_api: Box::new(::apis::NodeInfoApiClient::new(rc.clone())),
      oracle_api: Box::new(::apis::OracleApiClient::new(rc.clone())),
      transaction_api: Box::new(::apis::TransactionApiClient::new(rc.clone())),
    }
  }

  pub fn account_api(&self) -> &::apis::AccountApi{
    self.account_api.as_ref()
  }

  pub fn chain_api(&self) -> &::apis::ChainApi{
    self.chain_api.as_ref()
  }

  pub fn channel_api(&self) -> &::apis::ChannelApi{
    self.channel_api.as_ref()
  }

  pub fn contract_api(&self) -> &::apis::ContractApi{
    self.contract_api.as_ref()
  }

  pub fn debug_api(&self) -> &::apis::DebugApi{
    self.debug_api.as_ref()
  }

  pub fn external_api(&self) -> &::apis::ExternalApi{
    self.external_api.as_ref()
  }

  pub fn internal_api(&self) -> &::apis::InternalApi{
    self.internal_api.as_ref()
  }

  pub fn name_service_api(&self) -> &::apis::NameServiceApi{
    self.name_service_api.as_ref()
  }

  pub fn node_info_api(&self) -> &::apis::NodeInfoApi{
    self.node_info_api.as_ref()
  }

  pub fn oracle_api(&self) -> &::apis::OracleApi{
    self.oracle_api.as_ref()
  }

  pub fn transaction_api(&self) -> &::apis::TransactionApi{
    self.transaction_api.as_ref()
  }


}
