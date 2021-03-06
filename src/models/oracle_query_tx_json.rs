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
pub struct OracleQueryTxJson {
  #[serde(rename = "oracle_id")]
  oracle_id: ::models::EncodedHash,
  #[serde(rename = "query")]
  query: String,
  #[serde(rename = "query_fee")]
  query_fee: i64,
  #[serde(rename = "query_ttl")]
  query_ttl: ::models::Ttl,
  #[serde(rename = "response_ttl")]
  response_ttl: ::models::RelativeTtl,
  #[serde(rename = "fee")]
  fee: i64,
  #[serde(rename = "ttl")]
  ttl: Option<i64>,
  #[serde(rename = "sender_id")]
  sender_id: ::models::EncodedHash,
  /// Sender nonce
  #[serde(rename = "nonce")]
  nonce: Option<i64>,
  #[serde(rename = "version")]
  version: i64,
  #[serde(rename = "type")]
  _type: String
}

impl OracleQueryTxJson {
  pub fn new(oracle_id: ::models::EncodedHash, query: String, query_fee: i64, query_ttl: ::models::Ttl, response_ttl: ::models::RelativeTtl, fee: i64, sender_id: ::models::EncodedHash, version: i64, _type: String) -> OracleQueryTxJson {
    OracleQueryTxJson {
      oracle_id: oracle_id,
      query: query,
      query_fee: query_fee,
      query_ttl: query_ttl,
      response_ttl: response_ttl,
      fee: fee,
      ttl: None,
      sender_id: sender_id,
      nonce: None,
      version: version,
      _type: _type
    }
  }

  pub fn set_oracle_id(&mut self, oracle_id: ::models::EncodedHash) {
    self.oracle_id = oracle_id;
  }

  pub fn with_oracle_id(mut self, oracle_id: ::models::EncodedHash) -> OracleQueryTxJson {
    self.oracle_id = oracle_id;
    self
  }

  pub fn oracle_id(&self) -> &::models::EncodedHash {
    &self.oracle_id
  }


  pub fn set_query(&mut self, query: String) {
    self.query = query;
  }

  pub fn with_query(mut self, query: String) -> OracleQueryTxJson {
    self.query = query;
    self
  }

  pub fn query(&self) -> &String {
    &self.query
  }


  pub fn set_query_fee(&mut self, query_fee: i64) {
    self.query_fee = query_fee;
  }

  pub fn with_query_fee(mut self, query_fee: i64) -> OracleQueryTxJson {
    self.query_fee = query_fee;
    self
  }

  pub fn query_fee(&self) -> &i64 {
    &self.query_fee
  }


  pub fn set_query_ttl(&mut self, query_ttl: ::models::Ttl) {
    self.query_ttl = query_ttl;
  }

  pub fn with_query_ttl(mut self, query_ttl: ::models::Ttl) -> OracleQueryTxJson {
    self.query_ttl = query_ttl;
    self
  }

  pub fn query_ttl(&self) -> &::models::Ttl {
    &self.query_ttl
  }


  pub fn set_response_ttl(&mut self, response_ttl: ::models::RelativeTtl) {
    self.response_ttl = response_ttl;
  }

  pub fn with_response_ttl(mut self, response_ttl: ::models::RelativeTtl) -> OracleQueryTxJson {
    self.response_ttl = response_ttl;
    self
  }

  pub fn response_ttl(&self) -> &::models::RelativeTtl {
    &self.response_ttl
  }


  pub fn set_fee(&mut self, fee: i64) {
    self.fee = fee;
  }

  pub fn with_fee(mut self, fee: i64) -> OracleQueryTxJson {
    self.fee = fee;
    self
  }

  pub fn fee(&self) -> &i64 {
    &self.fee
  }


  pub fn set_ttl(&mut self, ttl: i64) {
    self.ttl = Some(ttl);
  }

  pub fn with_ttl(mut self, ttl: i64) -> OracleQueryTxJson {
    self.ttl = Some(ttl);
    self
  }

  pub fn ttl(&self) -> Option<&i64> {
    self.ttl.as_ref()
  }

  pub fn reset_ttl(&mut self) {
    self.ttl = None;
  }

  pub fn set_sender_id(&mut self, sender_id: ::models::EncodedHash) {
    self.sender_id = sender_id;
  }

  pub fn with_sender_id(mut self, sender_id: ::models::EncodedHash) -> OracleQueryTxJson {
    self.sender_id = sender_id;
    self
  }

  pub fn sender_id(&self) -> &::models::EncodedHash {
    &self.sender_id
  }


  pub fn set_nonce(&mut self, nonce: i64) {
    self.nonce = Some(nonce);
  }

  pub fn with_nonce(mut self, nonce: i64) -> OracleQueryTxJson {
    self.nonce = Some(nonce);
    self
  }

  pub fn nonce(&self) -> Option<&i64> {
    self.nonce.as_ref()
  }

  pub fn reset_nonce(&mut self) {
    self.nonce = None;
  }

  pub fn set_version(&mut self, version: i64) {
    self.version = version;
  }

  pub fn with_version(mut self, version: i64) -> OracleQueryTxJson {
    self.version = version;
    self
  }

  pub fn version(&self) -> &i64 {
    &self.version
  }


  pub fn set__type(&mut self, _type: String) {
    self._type = _type;
  }

  pub fn with__type(mut self, _type: String) -> OracleQueryTxJson {
    self._type = _type;
    self
  }

  pub fn _type(&self) -> &String {
    &self._type
  }


}



