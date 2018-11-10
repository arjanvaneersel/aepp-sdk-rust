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
pub struct OffChainDeposit {
  #[serde(rename = "op")]
  op: String,
  /// Depositor of tokens
  #[serde(rename = "from")]
  from: ::models::EncodedHash,
  /// Amount of tokens to deposit
  #[serde(rename = "am")]
  am: i64
}

impl OffChainDeposit {
  pub fn new(op: String, from: ::models::EncodedHash, am: i64) -> OffChainDeposit {
    OffChainDeposit {
      op: op,
      from: from,
      am: am
    }
  }

  pub fn set_op(&mut self, op: String) {
    self.op = op;
  }

  pub fn with_op(mut self, op: String) -> OffChainDeposit {
    self.op = op;
    self
  }

  pub fn op(&self) -> &String {
    &self.op
  }


  pub fn set_from(&mut self, from: ::models::EncodedHash) {
    self.from = from;
  }

  pub fn with_from(mut self, from: ::models::EncodedHash) -> OffChainDeposit {
    self.from = from;
    self
  }

  pub fn from(&self) -> &::models::EncodedHash {
    &self.from
  }


  pub fn set_am(&mut self, am: i64) {
    self.am = am;
  }

  pub fn with_am(mut self, am: i64) -> OffChainDeposit {
    self.am = am;
    self
  }

  pub fn am(&self) -> &i64 {
    &self.am
  }


}


