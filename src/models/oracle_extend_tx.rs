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
pub struct OracleExtendTx {
  #[serde(rename = "fee")]
  fee: i64,
  #[serde(rename = "oracle_ttl")]
  oracle_ttl: ::models::RelativeTtl,
  #[serde(rename = "oracle_id")]
  oracle_id: ::models::EncodedHash,
  #[serde(rename = "nonce")]
  nonce: Option<i64>,
  #[serde(rename = "ttl")]
  ttl: Option<i64>
}

impl OracleExtendTx {
  pub fn new(fee: i64, oracle_ttl: ::models::RelativeTtl, oracle_id: ::models::EncodedHash) -> OracleExtendTx {
    OracleExtendTx {
      fee: fee,
      oracle_ttl: oracle_ttl,
      oracle_id: oracle_id,
      nonce: None,
      ttl: None
    }
  }

  pub fn set_fee(&mut self, fee: i64) {
    self.fee = fee;
  }

  pub fn with_fee(mut self, fee: i64) -> OracleExtendTx {
    self.fee = fee;
    self
  }

  pub fn fee(&self) -> &i64 {
    &self.fee
  }


  pub fn set_oracle_ttl(&mut self, oracle_ttl: ::models::RelativeTtl) {
    self.oracle_ttl = oracle_ttl;
  }

  pub fn with_oracle_ttl(mut self, oracle_ttl: ::models::RelativeTtl) -> OracleExtendTx {
    self.oracle_ttl = oracle_ttl;
    self
  }

  pub fn oracle_ttl(&self) -> &::models::RelativeTtl {
    &self.oracle_ttl
  }


  pub fn set_oracle_id(&mut self, oracle_id: ::models::EncodedHash) {
    self.oracle_id = oracle_id;
  }

  pub fn with_oracle_id(mut self, oracle_id: ::models::EncodedHash) -> OracleExtendTx {
    self.oracle_id = oracle_id;
    self
  }

  pub fn oracle_id(&self) -> &::models::EncodedHash {
    &self.oracle_id
  }


  pub fn set_nonce(&mut self, nonce: i64) {
    self.nonce = Some(nonce);
  }

  pub fn with_nonce(mut self, nonce: i64) -> OracleExtendTx {
    self.nonce = Some(nonce);
    self
  }

  pub fn nonce(&self) -> Option<&i64> {
    self.nonce.as_ref()
  }

  pub fn reset_nonce(&mut self) {
    self.nonce = None;
  }

  pub fn set_ttl(&mut self, ttl: i64) {
    self.ttl = Some(ttl);
  }

  pub fn with_ttl(mut self, ttl: i64) -> OracleExtendTx {
    self.ttl = Some(ttl);
    self
  }

  pub fn ttl(&self) -> Option<&i64> {
    self.ttl.as_ref()
  }

  pub fn reset_ttl(&mut self) {
    self.ttl = None;
  }

}


