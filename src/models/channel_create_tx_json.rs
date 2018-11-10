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
pub struct ChannelCreateTxJson {
  #[serde(rename = "initiator_id")]
  initiator_id: ::models::EncodedHash,
  #[serde(rename = "initiator_amount")]
  initiator_amount: i64,
  #[serde(rename = "responder_id")]
  responder_id: ::models::EncodedHash,
  #[serde(rename = "responder_amount")]
  responder_amount: i64,
  #[serde(rename = "push_amount")]
  push_amount: i64,
  #[serde(rename = "channel_reserve")]
  channel_reserve: i64,
  #[serde(rename = "lock_period")]
  lock_period: i64,
  #[serde(rename = "ttl")]
  ttl: Option<i64>,
  #[serde(rename = "fee")]
  fee: i64,
  #[serde(rename = "nonce")]
  nonce: Option<i64>,
  /// Root hash of the channel's internal state tree
  #[serde(rename = "state_hash")]
  state_hash: ::models::EncodedHash,
  #[serde(rename = "version")]
  version: i64,
  #[serde(rename = "type")]
  _type: String
}

impl ChannelCreateTxJson {
  pub fn new(initiator_id: ::models::EncodedHash, initiator_amount: i64, responder_id: ::models::EncodedHash, responder_amount: i64, push_amount: i64, channel_reserve: i64, lock_period: i64, fee: i64, state_hash: ::models::EncodedHash, version: i64, _type: String) -> ChannelCreateTxJson {
    ChannelCreateTxJson {
      initiator_id: initiator_id,
      initiator_amount: initiator_amount,
      responder_id: responder_id,
      responder_amount: responder_amount,
      push_amount: push_amount,
      channel_reserve: channel_reserve,
      lock_period: lock_period,
      ttl: None,
      fee: fee,
      nonce: None,
      state_hash: state_hash,
      version: version,
      _type: _type
    }
  }

  pub fn set_initiator_id(&mut self, initiator_id: ::models::EncodedHash) {
    self.initiator_id = initiator_id;
  }

  pub fn with_initiator_id(mut self, initiator_id: ::models::EncodedHash) -> ChannelCreateTxJson {
    self.initiator_id = initiator_id;
    self
  }

  pub fn initiator_id(&self) -> &::models::EncodedHash {
    &self.initiator_id
  }


  pub fn set_initiator_amount(&mut self, initiator_amount: i64) {
    self.initiator_amount = initiator_amount;
  }

  pub fn with_initiator_amount(mut self, initiator_amount: i64) -> ChannelCreateTxJson {
    self.initiator_amount = initiator_amount;
    self
  }

  pub fn initiator_amount(&self) -> &i64 {
    &self.initiator_amount
  }


  pub fn set_responder_id(&mut self, responder_id: ::models::EncodedHash) {
    self.responder_id = responder_id;
  }

  pub fn with_responder_id(mut self, responder_id: ::models::EncodedHash) -> ChannelCreateTxJson {
    self.responder_id = responder_id;
    self
  }

  pub fn responder_id(&self) -> &::models::EncodedHash {
    &self.responder_id
  }


  pub fn set_responder_amount(&mut self, responder_amount: i64) {
    self.responder_amount = responder_amount;
  }

  pub fn with_responder_amount(mut self, responder_amount: i64) -> ChannelCreateTxJson {
    self.responder_amount = responder_amount;
    self
  }

  pub fn responder_amount(&self) -> &i64 {
    &self.responder_amount
  }


  pub fn set_push_amount(&mut self, push_amount: i64) {
    self.push_amount = push_amount;
  }

  pub fn with_push_amount(mut self, push_amount: i64) -> ChannelCreateTxJson {
    self.push_amount = push_amount;
    self
  }

  pub fn push_amount(&self) -> &i64 {
    &self.push_amount
  }


  pub fn set_channel_reserve(&mut self, channel_reserve: i64) {
    self.channel_reserve = channel_reserve;
  }

  pub fn with_channel_reserve(mut self, channel_reserve: i64) -> ChannelCreateTxJson {
    self.channel_reserve = channel_reserve;
    self
  }

  pub fn channel_reserve(&self) -> &i64 {
    &self.channel_reserve
  }


  pub fn set_lock_period(&mut self, lock_period: i64) {
    self.lock_period = lock_period;
  }

  pub fn with_lock_period(mut self, lock_period: i64) -> ChannelCreateTxJson {
    self.lock_period = lock_period;
    self
  }

  pub fn lock_period(&self) -> &i64 {
    &self.lock_period
  }


  pub fn set_ttl(&mut self, ttl: i64) {
    self.ttl = Some(ttl);
  }

  pub fn with_ttl(mut self, ttl: i64) -> ChannelCreateTxJson {
    self.ttl = Some(ttl);
    self
  }

  pub fn ttl(&self) -> Option<&i64> {
    self.ttl.as_ref()
  }

  pub fn reset_ttl(&mut self) {
    self.ttl = None;
  }

  pub fn set_fee(&mut self, fee: i64) {
    self.fee = fee;
  }

  pub fn with_fee(mut self, fee: i64) -> ChannelCreateTxJson {
    self.fee = fee;
    self
  }

  pub fn fee(&self) -> &i64 {
    &self.fee
  }


  pub fn set_nonce(&mut self, nonce: i64) {
    self.nonce = Some(nonce);
  }

  pub fn with_nonce(mut self, nonce: i64) -> ChannelCreateTxJson {
    self.nonce = Some(nonce);
    self
  }

  pub fn nonce(&self) -> Option<&i64> {
    self.nonce.as_ref()
  }

  pub fn reset_nonce(&mut self) {
    self.nonce = None;
  }

  pub fn set_state_hash(&mut self, state_hash: ::models::EncodedHash) {
    self.state_hash = state_hash;
  }

  pub fn with_state_hash(mut self, state_hash: ::models::EncodedHash) -> ChannelCreateTxJson {
    self.state_hash = state_hash;
    self
  }

  pub fn state_hash(&self) -> &::models::EncodedHash {
    &self.state_hash
  }


  pub fn set_version(&mut self, version: i64) {
    self.version = version;
  }

  pub fn with_version(mut self, version: i64) -> ChannelCreateTxJson {
    self.version = version;
    self
  }

  pub fn version(&self) -> &i64 {
    &self.version
  }


  pub fn set__type(&mut self, _type: String) {
    self._type = _type;
  }

  pub fn with__type(mut self, _type: String) -> ChannelCreateTxJson {
    self._type = _type;
    self
  }

  pub fn _type(&self) -> &String {
    &self._type
  }


}



