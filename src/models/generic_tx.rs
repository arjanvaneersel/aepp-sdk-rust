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
pub struct GenericTx {
  #[serde(rename = "version")]
  version: i64,
  #[serde(rename = "type")]
  _type: String
}

impl GenericTx {
  pub fn new(version: i64, _type: String) -> GenericTx {
    GenericTx {
      version: version,
      _type: _type
    }
  }

  pub fn set_version(&mut self, version: i64) {
    self.version = version;
  }

  pub fn with_version(mut self, version: i64) -> GenericTx {
    self.version = version;
    self
  }

  pub fn version(&self) -> &i64 {
    &self.version
  }


  pub fn set__type(&mut self, _type: String) {
    self._type = _type;
  }

  pub fn with__type(mut self, _type: String) -> GenericTx {
    self._type = _type;
    self
  }

  pub fn _type(&self) -> &String {
    &self._type
  }


}


