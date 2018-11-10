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
pub struct ContractCreateCompute {
  /// Contract owner pub_key
  #[serde(rename = "owner_id")]
  owner_id: String,
  /// Owner's nonce
  #[serde(rename = "nonce")]
  nonce: Option<i32>,
  /// Contract's code
  #[serde(rename = "code")]
  code: String,
  /// Virtual machine's version
  #[serde(rename = "vm_version")]
  vm_version: i32,
  /// Deposit
  #[serde(rename = "deposit")]
  deposit: i32,
  /// Amount
  #[serde(rename = "amount")]
  amount: i32,
  /// Contract gas
  #[serde(rename = "gas")]
  gas: i32,
  /// Gas price
  #[serde(rename = "gas_price")]
  gas_price: i32,
  /// Transaction fee
  #[serde(rename = "fee")]
  fee: i32,
  /// Transaction TTL
  #[serde(rename = "ttl")]
  ttl: Option<i32>,
  /// Contract call data init function arguments (deprecated, use call)
  #[serde(rename = "arguments")]
  arguments: Option<String>,
  /// Source code for a contract with a function __call() = init(args)
  #[serde(rename = "call")]
  call: Option<String>
}

impl ContractCreateCompute {
  pub fn new(owner_id: String, code: String, vm_version: i32, deposit: i32, amount: i32, gas: i32, gas_price: i32, fee: i32) -> ContractCreateCompute {
    ContractCreateCompute {
      owner_id: owner_id,
      nonce: None,
      code: code,
      vm_version: vm_version,
      deposit: deposit,
      amount: amount,
      gas: gas,
      gas_price: gas_price,
      fee: fee,
      ttl: None,
      arguments: None,
      call: None
    }
  }

  pub fn set_owner_id(&mut self, owner_id: String) {
    self.owner_id = owner_id;
  }

  pub fn with_owner_id(mut self, owner_id: String) -> ContractCreateCompute {
    self.owner_id = owner_id;
    self
  }

  pub fn owner_id(&self) -> &String {
    &self.owner_id
  }


  pub fn set_nonce(&mut self, nonce: i32) {
    self.nonce = Some(nonce);
  }

  pub fn with_nonce(mut self, nonce: i32) -> ContractCreateCompute {
    self.nonce = Some(nonce);
    self
  }

  pub fn nonce(&self) -> Option<&i32> {
    self.nonce.as_ref()
  }

  pub fn reset_nonce(&mut self) {
    self.nonce = None;
  }

  pub fn set_code(&mut self, code: String) {
    self.code = code;
  }

  pub fn with_code(mut self, code: String) -> ContractCreateCompute {
    self.code = code;
    self
  }

  pub fn code(&self) -> &String {
    &self.code
  }


  pub fn set_vm_version(&mut self, vm_version: i32) {
    self.vm_version = vm_version;
  }

  pub fn with_vm_version(mut self, vm_version: i32) -> ContractCreateCompute {
    self.vm_version = vm_version;
    self
  }

  pub fn vm_version(&self) -> &i32 {
    &self.vm_version
  }


  pub fn set_deposit(&mut self, deposit: i32) {
    self.deposit = deposit;
  }

  pub fn with_deposit(mut self, deposit: i32) -> ContractCreateCompute {
    self.deposit = deposit;
    self
  }

  pub fn deposit(&self) -> &i32 {
    &self.deposit
  }


  pub fn set_amount(&mut self, amount: i32) {
    self.amount = amount;
  }

  pub fn with_amount(mut self, amount: i32) -> ContractCreateCompute {
    self.amount = amount;
    self
  }

  pub fn amount(&self) -> &i32 {
    &self.amount
  }


  pub fn set_gas(&mut self, gas: i32) {
    self.gas = gas;
  }

  pub fn with_gas(mut self, gas: i32) -> ContractCreateCompute {
    self.gas = gas;
    self
  }

  pub fn gas(&self) -> &i32 {
    &self.gas
  }


  pub fn set_gas_price(&mut self, gas_price: i32) {
    self.gas_price = gas_price;
  }

  pub fn with_gas_price(mut self, gas_price: i32) -> ContractCreateCompute {
    self.gas_price = gas_price;
    self
  }

  pub fn gas_price(&self) -> &i32 {
    &self.gas_price
  }


  pub fn set_fee(&mut self, fee: i32) {
    self.fee = fee;
  }

  pub fn with_fee(mut self, fee: i32) -> ContractCreateCompute {
    self.fee = fee;
    self
  }

  pub fn fee(&self) -> &i32 {
    &self.fee
  }


  pub fn set_ttl(&mut self, ttl: i32) {
    self.ttl = Some(ttl);
  }

  pub fn with_ttl(mut self, ttl: i32) -> ContractCreateCompute {
    self.ttl = Some(ttl);
    self
  }

  pub fn ttl(&self) -> Option<&i32> {
    self.ttl.as_ref()
  }

  pub fn reset_ttl(&mut self) {
    self.ttl = None;
  }

  pub fn set_arguments(&mut self, arguments: String) {
    self.arguments = Some(arguments);
  }

  pub fn with_arguments(mut self, arguments: String) -> ContractCreateCompute {
    self.arguments = Some(arguments);
    self
  }

  pub fn arguments(&self) -> Option<&String> {
    self.arguments.as_ref()
  }

  pub fn reset_arguments(&mut self) {
    self.arguments = None;
  }

  pub fn set_call(&mut self, call: String) {
    self.call = Some(call);
  }

  pub fn with_call(mut self, call: String) -> ContractCreateCompute {
    self.call = Some(call);
    self
  }

  pub fn call(&self) -> Option<&String> {
    self.call.as_ref()
  }

  pub fn reset_call(&mut self) {
    self.call = None;
  }

}


