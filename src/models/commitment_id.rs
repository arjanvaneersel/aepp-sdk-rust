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
pub struct CommitmentId {
  #[serde(rename = "commitment_id")]
  commitment_id: ::models::EncodedHash
}

impl CommitmentId {
  pub fn new(commitment_id: ::models::EncodedHash) -> CommitmentId {
    CommitmentId {
      commitment_id: commitment_id
    }
  }

  pub fn set_commitment_id(&mut self, commitment_id: ::models::EncodedHash) {
    self.commitment_id = commitment_id;
  }

  pub fn with_commitment_id(mut self, commitment_id: ::models::EncodedHash) -> CommitmentId {
    self.commitment_id = commitment_id;
    self
  }

  pub fn commitment_id(&self) -> &::models::EncodedHash {
    &self.commitment_id
  }


}



