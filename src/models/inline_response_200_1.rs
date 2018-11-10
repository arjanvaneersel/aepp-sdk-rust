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
pub struct InlineResponse2001 {
  /// Height
  #[serde(rename = "height")]
  height: Option<i64>
}

impl InlineResponse2001 {
  pub fn new() -> InlineResponse2001 {
    InlineResponse2001 {
      height: None
    }
  }

  pub fn set_height(&mut self, height: i64) {
    self.height = Some(height);
  }

  pub fn with_height(mut self, height: i64) -> InlineResponse2001 {
    self.height = Some(height);
    self
  }

  pub fn height(&self) -> Option<&i64> {
    self.height.as_ref()
  }

  pub fn reset_height(&mut self) {
    self.height = None;
  }

}



