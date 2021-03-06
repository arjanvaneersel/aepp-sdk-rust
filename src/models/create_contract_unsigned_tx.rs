/* 
 * Aeternity Epoch
 *
 * This is the [Aeternity](https://www.aeternity.com/) Epoch API.
 *
 * OpenAPI spec version: 0.25.0
 * Contact: apiteam@aeternity.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateContractUnsignedTx {
  /// Unsigned transaction object
  #[serde(rename = "tx")]
  tx: ::models::EncodedHash,
  /// Address of the contract to be created
  #[serde(rename = "contract_id")]
  contract_id: ::models::EncodedHash
}

impl CreateContractUnsignedTx {
  pub fn new(tx: ::models::EncodedHash, contract_id: ::models::EncodedHash) -> CreateContractUnsignedTx {
    CreateContractUnsignedTx {
      tx: tx,
      contract_id: contract_id
    }
  }

  pub fn set_tx(&mut self, tx: ::models::EncodedHash) {
    self.tx = tx;
  }

  pub fn with_tx(mut self, tx: ::models::EncodedHash) -> CreateContractUnsignedTx {
    self.tx = tx;
    self
  }

  pub fn tx(&self) -> &::models::EncodedHash {
    &self.tx
  }


  pub fn set_contract_id(&mut self, contract_id: ::models::EncodedHash) {
    self.contract_id = contract_id;
  }

  pub fn with_contract_id(mut self, contract_id: ::models::EncodedHash) -> CreateContractUnsignedTx {
    self.contract_id = contract_id;
    self
  }

  pub fn contract_id(&self) -> &::models::EncodedHash {
    &self.contract_id
  }


}



