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
pub struct ContractCallObject {
  #[serde(rename = "caller_id")]
  caller_id: ::models::EncodedHash,
  #[serde(rename = "caller_nonce")]
  caller_nonce: i32,
  #[serde(rename = "height")]
  height: i32,
  #[serde(rename = "contract_id")]
  contract_id: ::models::EncodedHash,
  #[serde(rename = "gas_price")]
  gas_price: i32,
  #[serde(rename = "gas_used")]
  gas_used: i32,
  #[serde(rename = "return_value")]
  return_value: ::models::EncodedByteArray,
  /// The status of the call 'ok | error | revert'.
  #[serde(rename = "return_type")]
  return_type: String
}

impl ContractCallObject {
  pub fn new(caller_id: ::models::EncodedHash, caller_nonce: i32, height: i32, contract_id: ::models::EncodedHash, gas_price: i32, gas_used: i32, return_value: ::models::EncodedByteArray, return_type: String) -> ContractCallObject {
    ContractCallObject {
      caller_id: caller_id,
      caller_nonce: caller_nonce,
      height: height,
      contract_id: contract_id,
      gas_price: gas_price,
      gas_used: gas_used,
      return_value: return_value,
      return_type: return_type
    }
  }

  pub fn set_caller_id(&mut self, caller_id: ::models::EncodedHash) {
    self.caller_id = caller_id;
  }

  pub fn with_caller_id(mut self, caller_id: ::models::EncodedHash) -> ContractCallObject {
    self.caller_id = caller_id;
    self
  }

  pub fn caller_id(&self) -> &::models::EncodedHash {
    &self.caller_id
  }


  pub fn set_caller_nonce(&mut self, caller_nonce: i32) {
    self.caller_nonce = caller_nonce;
  }

  pub fn with_caller_nonce(mut self, caller_nonce: i32) -> ContractCallObject {
    self.caller_nonce = caller_nonce;
    self
  }

  pub fn caller_nonce(&self) -> &i32 {
    &self.caller_nonce
  }


  pub fn set_height(&mut self, height: i32) {
    self.height = height;
  }

  pub fn with_height(mut self, height: i32) -> ContractCallObject {
    self.height = height;
    self
  }

  pub fn height(&self) -> &i32 {
    &self.height
  }


  pub fn set_contract_id(&mut self, contract_id: ::models::EncodedHash) {
    self.contract_id = contract_id;
  }

  pub fn with_contract_id(mut self, contract_id: ::models::EncodedHash) -> ContractCallObject {
    self.contract_id = contract_id;
    self
  }

  pub fn contract_id(&self) -> &::models::EncodedHash {
    &self.contract_id
  }


  pub fn set_gas_price(&mut self, gas_price: i32) {
    self.gas_price = gas_price;
  }

  pub fn with_gas_price(mut self, gas_price: i32) -> ContractCallObject {
    self.gas_price = gas_price;
    self
  }

  pub fn gas_price(&self) -> &i32 {
    &self.gas_price
  }


  pub fn set_gas_used(&mut self, gas_used: i32) {
    self.gas_used = gas_used;
  }

  pub fn with_gas_used(mut self, gas_used: i32) -> ContractCallObject {
    self.gas_used = gas_used;
    self
  }

  pub fn gas_used(&self) -> &i32 {
    &self.gas_used
  }


  pub fn set_return_value(&mut self, return_value: ::models::EncodedByteArray) {
    self.return_value = return_value;
  }

  pub fn with_return_value(mut self, return_value: ::models::EncodedByteArray) -> ContractCallObject {
    self.return_value = return_value;
    self
  }

  pub fn return_value(&self) -> &::models::EncodedByteArray {
    &self.return_value
  }


  pub fn set_return_type(&mut self, return_type: String) {
    self.return_type = return_type;
  }

  pub fn with_return_type(mut self, return_type: String) -> ContractCallObject {
    self.return_type = return_type;
    self
  }

  pub fn return_type(&self) -> &String {
    &self.return_type
  }


}



