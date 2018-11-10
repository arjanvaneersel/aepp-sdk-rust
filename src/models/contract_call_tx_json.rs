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
pub struct ContractCallTxJson {
  /// Contract caller pub_key
  #[serde(rename = "caller_id")]
  caller_id: ::models::EncodedHash,
  /// Caller's nonce
  #[serde(rename = "nonce")]
  nonce: Option<i32>,
  /// Contract's pub_key
  #[serde(rename = "contract_id")]
  contract_id: ::models::EncodedHash,
  /// Virtual machine's version
  #[serde(rename = "vm_version")]
  vm_version: i32,
  /// Transaction fee
  #[serde(rename = "fee")]
  fee: i32,
  /// Transaction TTL
  #[serde(rename = "ttl")]
  ttl: Option<i32>,
  /// Amount
  #[serde(rename = "amount")]
  amount: i32,
  /// Contract gas
  #[serde(rename = "gas")]
  gas: i32,
  /// Gas price
  #[serde(rename = "gas_price")]
  gas_price: i32,
  /// Contract call data
  #[serde(rename = "call_data")]
  call_data: ::models::EncodedByteArray,
  #[serde(rename = "version")]
  version: i64,
  #[serde(rename = "type")]
  _type: String
}

impl ContractCallTxJson {
  pub fn new(caller_id: ::models::EncodedHash, contract_id: ::models::EncodedHash, vm_version: i32, fee: i32, amount: i32, gas: i32, gas_price: i32, call_data: ::models::EncodedByteArray, version: i64, _type: String) -> ContractCallTxJson {
    ContractCallTxJson {
      caller_id: caller_id,
      nonce: None,
      contract_id: contract_id,
      vm_version: vm_version,
      fee: fee,
      ttl: None,
      amount: amount,
      gas: gas,
      gas_price: gas_price,
      call_data: call_data,
      version: version,
      _type: _type
    }
  }

  pub fn set_caller_id(&mut self, caller_id: ::models::EncodedHash) {
    self.caller_id = caller_id;
  }

  pub fn with_caller_id(mut self, caller_id: ::models::EncodedHash) -> ContractCallTxJson {
    self.caller_id = caller_id;
    self
  }

  pub fn caller_id(&self) -> &::models::EncodedHash {
    &self.caller_id
  }


  pub fn set_nonce(&mut self, nonce: i32) {
    self.nonce = Some(nonce);
  }

  pub fn with_nonce(mut self, nonce: i32) -> ContractCallTxJson {
    self.nonce = Some(nonce);
    self
  }

  pub fn nonce(&self) -> Option<&i32> {
    self.nonce.as_ref()
  }

  pub fn reset_nonce(&mut self) {
    self.nonce = None;
  }

  pub fn set_contract_id(&mut self, contract_id: ::models::EncodedHash) {
    self.contract_id = contract_id;
  }

  pub fn with_contract_id(mut self, contract_id: ::models::EncodedHash) -> ContractCallTxJson {
    self.contract_id = contract_id;
    self
  }

  pub fn contract_id(&self) -> &::models::EncodedHash {
    &self.contract_id
  }


  pub fn set_vm_version(&mut self, vm_version: i32) {
    self.vm_version = vm_version;
  }

  pub fn with_vm_version(mut self, vm_version: i32) -> ContractCallTxJson {
    self.vm_version = vm_version;
    self
  }

  pub fn vm_version(&self) -> &i32 {
    &self.vm_version
  }


  pub fn set_fee(&mut self, fee: i32) {
    self.fee = fee;
  }

  pub fn with_fee(mut self, fee: i32) -> ContractCallTxJson {
    self.fee = fee;
    self
  }

  pub fn fee(&self) -> &i32 {
    &self.fee
  }


  pub fn set_ttl(&mut self, ttl: i32) {
    self.ttl = Some(ttl);
  }

  pub fn with_ttl(mut self, ttl: i32) -> ContractCallTxJson {
    self.ttl = Some(ttl);
    self
  }

  pub fn ttl(&self) -> Option<&i32> {
    self.ttl.as_ref()
  }

  pub fn reset_ttl(&mut self) {
    self.ttl = None;
  }

  pub fn set_amount(&mut self, amount: i32) {
    self.amount = amount;
  }

  pub fn with_amount(mut self, amount: i32) -> ContractCallTxJson {
    self.amount = amount;
    self
  }

  pub fn amount(&self) -> &i32 {
    &self.amount
  }


  pub fn set_gas(&mut self, gas: i32) {
    self.gas = gas;
  }

  pub fn with_gas(mut self, gas: i32) -> ContractCallTxJson {
    self.gas = gas;
    self
  }

  pub fn gas(&self) -> &i32 {
    &self.gas
  }


  pub fn set_gas_price(&mut self, gas_price: i32) {
    self.gas_price = gas_price;
  }

  pub fn with_gas_price(mut self, gas_price: i32) -> ContractCallTxJson {
    self.gas_price = gas_price;
    self
  }

  pub fn gas_price(&self) -> &i32 {
    &self.gas_price
  }


  pub fn set_call_data(&mut self, call_data: ::models::EncodedByteArray) {
    self.call_data = call_data;
  }

  pub fn with_call_data(mut self, call_data: ::models::EncodedByteArray) -> ContractCallTxJson {
    self.call_data = call_data;
    self
  }

  pub fn call_data(&self) -> &::models::EncodedByteArray {
    &self.call_data
  }


  pub fn set_version(&mut self, version: i64) {
    self.version = version;
  }

  pub fn with_version(mut self, version: i64) -> ContractCallTxJson {
    self.version = version;
    self
  }

  pub fn version(&self) -> &i64 {
    &self.version
  }


  pub fn set__type(&mut self, _type: String) {
    self._type = _type;
  }

  pub fn with__type(mut self, _type: String) -> ContractCallTxJson {
    self._type = _type;
    self
  }

  pub fn _type(&self) -> &String {
    &self._type
  }


}



