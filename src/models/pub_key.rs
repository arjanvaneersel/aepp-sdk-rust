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
pub struct PubKey {
  #[serde(rename = "pub_key")]
  pub_key: String
}

impl PubKey {
  pub fn new(pub_key: String) -> PubKey {
    PubKey {
      pub_key: pub_key
    }
  }

  pub fn set_pub_key(&mut self, pub_key: String) {
    self.pub_key = pub_key;
  }

  pub fn with_pub_key(mut self, pub_key: String) -> PubKey {
    self.pub_key = pub_key;
    self
  }

  pub fn pub_key(&self) -> &String {
    &self.pub_key
  }


}


