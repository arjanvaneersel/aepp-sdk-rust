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
pub struct Status {
  #[serde(rename = "genesis_key_block_hash")]
  genesis_key_block_hash: ::models::EncodedHash,
  #[serde(rename = "solutions")]
  solutions: i64,
  #[serde(rename = "difficulty")]
  difficulty: f32,
  #[serde(rename = "syncing")]
  syncing: bool,
  #[serde(rename = "listening")]
  listening: bool,
  #[serde(rename = "protocols")]
  protocols: Vec<::models::Protocol>,
  #[serde(rename = "node_version")]
  node_version: String,
  #[serde(rename = "node_revision")]
  node_revision: String,
  #[serde(rename = "peer_count")]
  peer_count: i64,
  #[serde(rename = "pending_transactions_count")]
  pending_transactions_count: i64
}

impl Status {
  pub fn new(genesis_key_block_hash: ::models::EncodedHash, solutions: i64, difficulty: f32, syncing: bool, listening: bool, protocols: Vec<::models::Protocol>, node_version: String, node_revision: String, peer_count: i64, pending_transactions_count: i64) -> Status {
    Status {
      genesis_key_block_hash: genesis_key_block_hash,
      solutions: solutions,
      difficulty: difficulty,
      syncing: syncing,
      listening: listening,
      protocols: protocols,
      node_version: node_version,
      node_revision: node_revision,
      peer_count: peer_count,
      pending_transactions_count: pending_transactions_count
    }
  }

  pub fn set_genesis_key_block_hash(&mut self, genesis_key_block_hash: ::models::EncodedHash) {
    self.genesis_key_block_hash = genesis_key_block_hash;
  }

  pub fn with_genesis_key_block_hash(mut self, genesis_key_block_hash: ::models::EncodedHash) -> Status {
    self.genesis_key_block_hash = genesis_key_block_hash;
    self
  }

  pub fn genesis_key_block_hash(&self) -> &::models::EncodedHash {
    &self.genesis_key_block_hash
  }


  pub fn set_solutions(&mut self, solutions: i64) {
    self.solutions = solutions;
  }

  pub fn with_solutions(mut self, solutions: i64) -> Status {
    self.solutions = solutions;
    self
  }

  pub fn solutions(&self) -> &i64 {
    &self.solutions
  }


  pub fn set_difficulty(&mut self, difficulty: f32) {
    self.difficulty = difficulty;
  }

  pub fn with_difficulty(mut self, difficulty: f32) -> Status {
    self.difficulty = difficulty;
    self
  }

  pub fn difficulty(&self) -> &f32 {
    &self.difficulty
  }


  pub fn set_syncing(&mut self, syncing: bool) {
    self.syncing = syncing;
  }

  pub fn with_syncing(mut self, syncing: bool) -> Status {
    self.syncing = syncing;
    self
  }

  pub fn syncing(&self) -> &bool {
    &self.syncing
  }


  pub fn set_listening(&mut self, listening: bool) {
    self.listening = listening;
  }

  pub fn with_listening(mut self, listening: bool) -> Status {
    self.listening = listening;
    self
  }

  pub fn listening(&self) -> &bool {
    &self.listening
  }


  pub fn set_protocols(&mut self, protocols: Vec<::models::Protocol>) {
    self.protocols = protocols;
  }

  pub fn with_protocols(mut self, protocols: Vec<::models::Protocol>) -> Status {
    self.protocols = protocols;
    self
  }

  pub fn protocols(&self) -> &Vec<::models::Protocol> {
    &self.protocols
  }


  pub fn set_node_version(&mut self, node_version: String) {
    self.node_version = node_version;
  }

  pub fn with_node_version(mut self, node_version: String) -> Status {
    self.node_version = node_version;
    self
  }

  pub fn node_version(&self) -> &String {
    &self.node_version
  }


  pub fn set_node_revision(&mut self, node_revision: String) {
    self.node_revision = node_revision;
  }

  pub fn with_node_revision(mut self, node_revision: String) -> Status {
    self.node_revision = node_revision;
    self
  }

  pub fn node_revision(&self) -> &String {
    &self.node_revision
  }


  pub fn set_peer_count(&mut self, peer_count: i64) {
    self.peer_count = peer_count;
  }

  pub fn with_peer_count(mut self, peer_count: i64) -> Status {
    self.peer_count = peer_count;
    self
  }

  pub fn peer_count(&self) -> &i64 {
    &self.peer_count
  }


  pub fn set_pending_transactions_count(&mut self, pending_transactions_count: i64) {
    self.pending_transactions_count = pending_transactions_count;
  }

  pub fn with_pending_transactions_count(mut self, pending_transactions_count: i64) -> Status {
    self.pending_transactions_count = pending_transactions_count;
    self
  }

  pub fn pending_transactions_count(&self) -> &i64 {
    &self.pending_transactions_count
  }


}



