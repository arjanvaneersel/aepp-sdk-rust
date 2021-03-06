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
pub struct UnsignedTx {
  /// Unsigned transaction object
  #[serde(rename = "tx")]
  tx: ::models::EncodedHash
}

impl UnsignedTx {
  pub fn new(tx: ::models::EncodedHash) -> UnsignedTx {
    UnsignedTx {
      tx: tx
    }
  }

  pub fn set_tx(&mut self, tx: ::models::EncodedHash) {
    self.tx = tx;
  }

  pub fn with_tx(mut self, tx: ::models::EncodedHash) -> UnsignedTx {
    self.tx = tx;
    self
  }

  pub fn tx(&self) -> &::models::EncodedHash {
    &self.tx
  }


}



