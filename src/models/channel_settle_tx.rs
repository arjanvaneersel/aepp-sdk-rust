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
pub struct ChannelSettleTx {
  #[serde(rename = "channel_id")]
  channel_id: ::models::EncodedHash,
  #[serde(rename = "from_id")]
  from_id: ::models::EncodedHash,
  #[serde(rename = "initiator_amount_final")]
  initiator_amount_final: i64,
  #[serde(rename = "responder_amount_final")]
  responder_amount_final: i64,
  #[serde(rename = "ttl")]
  ttl: Option<i64>,
  #[serde(rename = "fee")]
  fee: i64,
  #[serde(rename = "nonce")]
  nonce: i64
}

impl ChannelSettleTx {
  pub fn new(channel_id: ::models::EncodedHash, from_id: ::models::EncodedHash, initiator_amount_final: i64, responder_amount_final: i64, fee: i64, nonce: i64) -> ChannelSettleTx {
    ChannelSettleTx {
      channel_id: channel_id,
      from_id: from_id,
      initiator_amount_final: initiator_amount_final,
      responder_amount_final: responder_amount_final,
      ttl: None,
      fee: fee,
      nonce: nonce
    }
  }

  pub fn set_channel_id(&mut self, channel_id: ::models::EncodedHash) {
    self.channel_id = channel_id;
  }

  pub fn with_channel_id(mut self, channel_id: ::models::EncodedHash) -> ChannelSettleTx {
    self.channel_id = channel_id;
    self
  }

  pub fn channel_id(&self) -> &::models::EncodedHash {
    &self.channel_id
  }


  pub fn set_from_id(&mut self, from_id: ::models::EncodedHash) {
    self.from_id = from_id;
  }

  pub fn with_from_id(mut self, from_id: ::models::EncodedHash) -> ChannelSettleTx {
    self.from_id = from_id;
    self
  }

  pub fn from_id(&self) -> &::models::EncodedHash {
    &self.from_id
  }


  pub fn set_initiator_amount_final(&mut self, initiator_amount_final: i64) {
    self.initiator_amount_final = initiator_amount_final;
  }

  pub fn with_initiator_amount_final(mut self, initiator_amount_final: i64) -> ChannelSettleTx {
    self.initiator_amount_final = initiator_amount_final;
    self
  }

  pub fn initiator_amount_final(&self) -> &i64 {
    &self.initiator_amount_final
  }


  pub fn set_responder_amount_final(&mut self, responder_amount_final: i64) {
    self.responder_amount_final = responder_amount_final;
  }

  pub fn with_responder_amount_final(mut self, responder_amount_final: i64) -> ChannelSettleTx {
    self.responder_amount_final = responder_amount_final;
    self
  }

  pub fn responder_amount_final(&self) -> &i64 {
    &self.responder_amount_final
  }


  pub fn set_ttl(&mut self, ttl: i64) {
    self.ttl = Some(ttl);
  }

  pub fn with_ttl(mut self, ttl: i64) -> ChannelSettleTx {
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

  pub fn with_fee(mut self, fee: i64) -> ChannelSettleTx {
    self.fee = fee;
    self
  }

  pub fn fee(&self) -> &i64 {
    &self.fee
  }


  pub fn set_nonce(&mut self, nonce: i64) {
    self.nonce = nonce;
  }

  pub fn with_nonce(mut self, nonce: i64) -> ChannelSettleTx {
    self.nonce = nonce;
    self
  }

  pub fn nonce(&self) -> &i64 {
    &self.nonce
  }


}



