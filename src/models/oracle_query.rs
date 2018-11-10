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
pub struct OracleQuery {
  #[serde(rename = "id")]
  id: ::models::EncodedHash,
  #[serde(rename = "sender_id")]
  sender_id: ::models::EncodedHash,
  #[serde(rename = "sender_nonce")]
  sender_nonce: i64,
  #[serde(rename = "oracle_id")]
  oracle_id: ::models::EncodedHash,
  #[serde(rename = "query")]
  query: String,
  #[serde(rename = "response")]
  response: String,
  #[serde(rename = "ttl")]
  ttl: i64,
  #[serde(rename = "response_ttl")]
  response_ttl: ::models::Ttl,
  #[serde(rename = "fee")]
  fee: i64
}

impl OracleQuery {
  pub fn new(id: ::models::EncodedHash, sender_id: ::models::EncodedHash, sender_nonce: i64, oracle_id: ::models::EncodedHash, query: String, response: String, ttl: i64, response_ttl: ::models::Ttl, fee: i64) -> OracleQuery {
    OracleQuery {
      id: id,
      sender_id: sender_id,
      sender_nonce: sender_nonce,
      oracle_id: oracle_id,
      query: query,
      response: response,
      ttl: ttl,
      response_ttl: response_ttl,
      fee: fee
    }
  }

  pub fn set_id(&mut self, id: ::models::EncodedHash) {
    self.id = id;
  }

  pub fn with_id(mut self, id: ::models::EncodedHash) -> OracleQuery {
    self.id = id;
    self
  }

  pub fn id(&self) -> &::models::EncodedHash {
    &self.id
  }


  pub fn set_sender_id(&mut self, sender_id: ::models::EncodedHash) {
    self.sender_id = sender_id;
  }

  pub fn with_sender_id(mut self, sender_id: ::models::EncodedHash) -> OracleQuery {
    self.sender_id = sender_id;
    self
  }

  pub fn sender_id(&self) -> &::models::EncodedHash {
    &self.sender_id
  }


  pub fn set_sender_nonce(&mut self, sender_nonce: i64) {
    self.sender_nonce = sender_nonce;
  }

  pub fn with_sender_nonce(mut self, sender_nonce: i64) -> OracleQuery {
    self.sender_nonce = sender_nonce;
    self
  }

  pub fn sender_nonce(&self) -> &i64 {
    &self.sender_nonce
  }


  pub fn set_oracle_id(&mut self, oracle_id: ::models::EncodedHash) {
    self.oracle_id = oracle_id;
  }

  pub fn with_oracle_id(mut self, oracle_id: ::models::EncodedHash) -> OracleQuery {
    self.oracle_id = oracle_id;
    self
  }

  pub fn oracle_id(&self) -> &::models::EncodedHash {
    &self.oracle_id
  }


  pub fn set_query(&mut self, query: String) {
    self.query = query;
  }

  pub fn with_query(mut self, query: String) -> OracleQuery {
    self.query = query;
    self
  }

  pub fn query(&self) -> &String {
    &self.query
  }


  pub fn set_response(&mut self, response: String) {
    self.response = response;
  }

  pub fn with_response(mut self, response: String) -> OracleQuery {
    self.response = response;
    self
  }

  pub fn response(&self) -> &String {
    &self.response
  }


  pub fn set_ttl(&mut self, ttl: i64) {
    self.ttl = ttl;
  }

  pub fn with_ttl(mut self, ttl: i64) -> OracleQuery {
    self.ttl = ttl;
    self
  }

  pub fn ttl(&self) -> &i64 {
    &self.ttl
  }


  pub fn set_response_ttl(&mut self, response_ttl: ::models::Ttl) {
    self.response_ttl = response_ttl;
  }

  pub fn with_response_ttl(mut self, response_ttl: ::models::Ttl) -> OracleQuery {
    self.response_ttl = response_ttl;
    self
  }

  pub fn response_ttl(&self) -> &::models::Ttl {
    &self.response_ttl
  }


  pub fn set_fee(&mut self, fee: i64) {
    self.fee = fee;
  }

  pub fn with_fee(mut self, fee: i64) -> OracleQuery {
    self.fee = fee;
    self
  }

  pub fn fee(&self) -> &i64 {
    &self.fee
  }


}


