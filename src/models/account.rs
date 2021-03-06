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
pub struct Account {
  /// Public key
  #[serde(rename = "id")]
  id: ::models::EncodedHash,
  /// Balance
  #[serde(rename = "balance")]
  balance: i64,
  /// Nonce
  #[serde(rename = "nonce")]
  nonce: i64
}

impl Account {
  pub fn new(id: ::models::EncodedHash, balance: i64, nonce: i64) -> Account {
    Account {
      id: id,
      balance: balance,
      nonce: nonce
    }
  }

  pub fn set_id(&mut self, id: ::models::EncodedHash) {
    self.id = id;
  }

  pub fn with_id(mut self, id: ::models::EncodedHash) -> Account {
    self.id = id;
    self
  }

  pub fn id(&self) -> &::models::EncodedHash {
    &self.id
  }


  pub fn set_balance(&mut self, balance: i64) {
    self.balance = balance;
  }

  pub fn with_balance(mut self, balance: i64) -> Account {
    self.balance = balance;
    self
  }

  pub fn balance(&self) -> &i64 {
    &self.balance
  }


  pub fn set_nonce(&mut self, nonce: i64) {
    self.nonce = nonce;
  }

  pub fn with_nonce(mut self, nonce: i64) -> Account {
    self.nonce = nonce;
    self
  }

  pub fn nonce(&self) -> &i64 {
    &self.nonce
  }


}



