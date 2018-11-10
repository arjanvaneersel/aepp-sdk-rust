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
pub struct RegisteredOracle {
  #[serde(rename = "id")]
  id: ::models::EncodedHash,
  #[serde(rename = "query_format")]
  query_format: String,
  #[serde(rename = "response_format")]
  response_format: String,
  #[serde(rename = "query_fee")]
  query_fee: i32,
  #[serde(rename = "ttl")]
  ttl: i64,
  #[serde(rename = "vm_version")]
  vm_version: i64
}

impl RegisteredOracle {
  pub fn new(id: ::models::EncodedHash, query_format: String, response_format: String, query_fee: i32, ttl: i64, vm_version: i64) -> RegisteredOracle {
    RegisteredOracle {
      id: id,
      query_format: query_format,
      response_format: response_format,
      query_fee: query_fee,
      ttl: ttl,
      vm_version: vm_version
    }
  }

  pub fn set_id(&mut self, id: ::models::EncodedHash) {
    self.id = id;
  }

  pub fn with_id(mut self, id: ::models::EncodedHash) -> RegisteredOracle {
    self.id = id;
    self
  }

  pub fn id(&self) -> &::models::EncodedHash {
    &self.id
  }


  pub fn set_query_format(&mut self, query_format: String) {
    self.query_format = query_format;
  }

  pub fn with_query_format(mut self, query_format: String) -> RegisteredOracle {
    self.query_format = query_format;
    self
  }

  pub fn query_format(&self) -> &String {
    &self.query_format
  }


  pub fn set_response_format(&mut self, response_format: String) {
    self.response_format = response_format;
  }

  pub fn with_response_format(mut self, response_format: String) -> RegisteredOracle {
    self.response_format = response_format;
    self
  }

  pub fn response_format(&self) -> &String {
    &self.response_format
  }


  pub fn set_query_fee(&mut self, query_fee: i32) {
    self.query_fee = query_fee;
  }

  pub fn with_query_fee(mut self, query_fee: i32) -> RegisteredOracle {
    self.query_fee = query_fee;
    self
  }

  pub fn query_fee(&self) -> &i32 {
    &self.query_fee
  }


  pub fn set_ttl(&mut self, ttl: i64) {
    self.ttl = ttl;
  }

  pub fn with_ttl(mut self, ttl: i64) -> RegisteredOracle {
    self.ttl = ttl;
    self
  }

  pub fn ttl(&self) -> &i64 {
    &self.ttl
  }


  pub fn set_vm_version(&mut self, vm_version: i64) {
    self.vm_version = vm_version;
  }

  pub fn with_vm_version(mut self, vm_version: i64) -> RegisteredOracle {
    self.vm_version = vm_version;
    self
  }

  pub fn vm_version(&self) -> &i64 {
    &self.vm_version
  }


}



