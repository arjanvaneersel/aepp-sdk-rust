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
pub struct NameUpdateTx {
  #[serde(rename = "name_id")]
  name_id: ::models::EncodedHash,
  #[serde(rename = "name_ttl")]
  name_ttl: i64,
  #[serde(rename = "pointers")]
  pointers: Vec<::models::NamePointer>,
  #[serde(rename = "client_ttl")]
  client_ttl: i64,
  #[serde(rename = "fee")]
  fee: i64,
  #[serde(rename = "ttl")]
  ttl: Option<i64>,
  #[serde(rename = "account_id")]
  account_id: ::models::EncodedHash,
  #[serde(rename = "nonce")]
  nonce: Option<i64>
}

impl NameUpdateTx {
  pub fn new(name_id: ::models::EncodedHash, name_ttl: i64, pointers: Vec<::models::NamePointer>, client_ttl: i64, fee: i64, account_id: ::models::EncodedHash) -> NameUpdateTx {
    NameUpdateTx {
      name_id: name_id,
      name_ttl: name_ttl,
      pointers: pointers,
      client_ttl: client_ttl,
      fee: fee,
      ttl: None,
      account_id: account_id,
      nonce: None
    }
  }

  pub fn set_name_id(&mut self, name_id: ::models::EncodedHash) {
    self.name_id = name_id;
  }

  pub fn with_name_id(mut self, name_id: ::models::EncodedHash) -> NameUpdateTx {
    self.name_id = name_id;
    self
  }

  pub fn name_id(&self) -> &::models::EncodedHash {
    &self.name_id
  }


  pub fn set_name_ttl(&mut self, name_ttl: i64) {
    self.name_ttl = name_ttl;
  }

  pub fn with_name_ttl(mut self, name_ttl: i64) -> NameUpdateTx {
    self.name_ttl = name_ttl;
    self
  }

  pub fn name_ttl(&self) -> &i64 {
    &self.name_ttl
  }


  pub fn set_pointers(&mut self, pointers: Vec<::models::NamePointer>) {
    self.pointers = pointers;
  }

  pub fn with_pointers(mut self, pointers: Vec<::models::NamePointer>) -> NameUpdateTx {
    self.pointers = pointers;
    self
  }

  pub fn pointers(&self) -> &Vec<::models::NamePointer> {
    &self.pointers
  }


  pub fn set_client_ttl(&mut self, client_ttl: i64) {
    self.client_ttl = client_ttl;
  }

  pub fn with_client_ttl(mut self, client_ttl: i64) -> NameUpdateTx {
    self.client_ttl = client_ttl;
    self
  }

  pub fn client_ttl(&self) -> &i64 {
    &self.client_ttl
  }


  pub fn set_fee(&mut self, fee: i64) {
    self.fee = fee;
  }

  pub fn with_fee(mut self, fee: i64) -> NameUpdateTx {
    self.fee = fee;
    self
  }

  pub fn fee(&self) -> &i64 {
    &self.fee
  }


  pub fn set_ttl(&mut self, ttl: i64) {
    self.ttl = Some(ttl);
  }

  pub fn with_ttl(mut self, ttl: i64) -> NameUpdateTx {
    self.ttl = Some(ttl);
    self
  }

  pub fn ttl(&self) -> Option<&i64> {
    self.ttl.as_ref()
  }

  pub fn reset_ttl(&mut self) {
    self.ttl = None;
  }

  pub fn set_account_id(&mut self, account_id: ::models::EncodedHash) {
    self.account_id = account_id;
  }

  pub fn with_account_id(mut self, account_id: ::models::EncodedHash) -> NameUpdateTx {
    self.account_id = account_id;
    self
  }

  pub fn account_id(&self) -> &::models::EncodedHash {
    &self.account_id
  }


  pub fn set_nonce(&mut self, nonce: i64) {
    self.nonce = Some(nonce);
  }

  pub fn with_nonce(mut self, nonce: i64) -> NameUpdateTx {
    self.nonce = Some(nonce);
    self
  }

  pub fn nonce(&self) -> Option<&i64> {
    self.nonce.as_ref()
  }

  pub fn reset_nonce(&mut self) {
    self.nonce = None;
  }

}



