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
pub struct Tx {
  #[serde(rename = "tx")]
  tx: String
}

impl Tx {
  pub fn new(tx: String) -> Tx {
    Tx {
      tx: tx
    }
  }

  pub fn set_tx(&mut self, tx: String) {
    self.tx = tx;
  }

  pub fn with_tx(mut self, tx: String) -> Tx {
    self.tx = tx;
    self
  }

  pub fn tx(&self) -> &String {
    &self.tx
  }


}


