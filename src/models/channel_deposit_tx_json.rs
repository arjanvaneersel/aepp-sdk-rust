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
pub struct ChannelDepositTxJson {
  #[serde(rename = "channel_id")]
  channel_id: ::models::EncodedHash,
  #[serde(rename = "from_id")]
  from_id: ::models::EncodedHash,
  #[serde(rename = "amount")]
  amount: i64,
  #[serde(rename = "ttl")]
  ttl: Option<i64>,
  #[serde(rename = "fee")]
  fee: i64,
  #[serde(rename = "nonce")]
  nonce: i64,
  /// Root hash of the channel's internal state tree after the deposit had been applied to it
  #[serde(rename = "state_hash")]
  state_hash: ::models::EncodedHash,
  /// Channel's next round
  #[serde(rename = "round")]
  round: i64,
  #[serde(rename = "version")]
  version: i64,
  #[serde(rename = "type")]
  _type: String
}

impl ChannelDepositTxJson {
  pub fn new(channel_id: ::models::EncodedHash, from_id: ::models::EncodedHash, amount: i64, fee: i64, nonce: i64, state_hash: ::models::EncodedHash, round: i64, version: i64, _type: String) -> ChannelDepositTxJson {
    ChannelDepositTxJson {
      channel_id: channel_id,
      from_id: from_id,
      amount: amount,
      ttl: None,
      fee: fee,
      nonce: nonce,
      state_hash: state_hash,
      round: round,
      version: version,
      _type: _type
    }
  }

  pub fn set_channel_id(&mut self, channel_id: ::models::EncodedHash) {
    self.channel_id = channel_id;
  }

  pub fn with_channel_id(mut self, channel_id: ::models::EncodedHash) -> ChannelDepositTxJson {
    self.channel_id = channel_id;
    self
  }

  pub fn channel_id(&self) -> &::models::EncodedHash {
    &self.channel_id
  }


  pub fn set_from_id(&mut self, from_id: ::models::EncodedHash) {
    self.from_id = from_id;
  }

  pub fn with_from_id(mut self, from_id: ::models::EncodedHash) -> ChannelDepositTxJson {
    self.from_id = from_id;
    self
  }

  pub fn from_id(&self) -> &::models::EncodedHash {
    &self.from_id
  }


  pub fn set_amount(&mut self, amount: i64) {
    self.amount = amount;
  }

  pub fn with_amount(mut self, amount: i64) -> ChannelDepositTxJson {
    self.amount = amount;
    self
  }

  pub fn amount(&self) -> &i64 {
    &self.amount
  }


  pub fn set_ttl(&mut self, ttl: i64) {
    self.ttl = Some(ttl);
  }

  pub fn with_ttl(mut self, ttl: i64) -> ChannelDepositTxJson {
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

  pub fn with_fee(mut self, fee: i64) -> ChannelDepositTxJson {
    self.fee = fee;
    self
  }

  pub fn fee(&self) -> &i64 {
    &self.fee
  }


  pub fn set_nonce(&mut self, nonce: i64) {
    self.nonce = nonce;
  }

  pub fn with_nonce(mut self, nonce: i64) -> ChannelDepositTxJson {
    self.nonce = nonce;
    self
  }

  pub fn nonce(&self) -> &i64 {
    &self.nonce
  }


  pub fn set_state_hash(&mut self, state_hash: ::models::EncodedHash) {
    self.state_hash = state_hash;
  }

  pub fn with_state_hash(mut self, state_hash: ::models::EncodedHash) -> ChannelDepositTxJson {
    self.state_hash = state_hash;
    self
  }

  pub fn state_hash(&self) -> &::models::EncodedHash {
    &self.state_hash
  }


  pub fn set_round(&mut self, round: i64) {
    self.round = round;
  }

  pub fn with_round(mut self, round: i64) -> ChannelDepositTxJson {
    self.round = round;
    self
  }

  pub fn round(&self) -> &i64 {
    &self.round
  }


  pub fn set_version(&mut self, version: i64) {
    self.version = version;
  }

  pub fn with_version(mut self, version: i64) -> ChannelDepositTxJson {
    self.version = version;
    self
  }

  pub fn version(&self) -> &i64 {
    &self.version
  }


  pub fn set__type(&mut self, _type: String) {
    self._type = _type;
  }

  pub fn with__type(mut self, _type: String) -> ChannelDepositTxJson {
    self._type = _type;
    self
  }

  pub fn _type(&self) -> &String {
    &self._type
  }


}



