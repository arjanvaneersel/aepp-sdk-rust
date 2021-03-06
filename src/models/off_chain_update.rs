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
pub struct OffChainUpdate {
  #[serde(rename = "op")]
  op: String
}

impl OffChainUpdate {
  pub fn new(op: String) -> OffChainUpdate {
    OffChainUpdate {
      op: op
    }
  }

  pub fn set_op(&mut self, op: String) {
    self.op = op;
  }

  pub fn with_op(mut self, op: String) -> OffChainUpdate {
    self.op = op;
    self
  }

  pub fn op(&self) -> &String {
    &self.op
  }


}



