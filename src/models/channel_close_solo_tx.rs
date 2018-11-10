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
pub struct ChannelCloseSoloTx {
  #[serde(rename = "channel_id")]
  channel_id: ::models::EncodedHash,
  #[serde(rename = "from_id")]
  from_id: ::models::EncodedHash,
  #[serde(rename = "payload")]
  payload: String,
  #[serde(rename = "ttl")]
  ttl: Option<i64>,
  #[serde(rename = "fee")]
  fee: i64,
  #[serde(rename = "nonce")]
  nonce: Option<i64>,
  /// Proof of inclusion containing information for closing the channel
  #[serde(rename = "poi")]
  poi: String
}

impl ChannelCloseSoloTx {
  pub fn new(channel_id: ::models::EncodedHash, from_id: ::models::EncodedHash, payload: String, fee: i64, poi: String) -> ChannelCloseSoloTx {
    ChannelCloseSoloTx {
      channel_id: channel_id,
      from_id: from_id,
      payload: payload,
      ttl: None,
      fee: fee,
      nonce: None,
      poi: poi
    }
  }

  pub fn set_channel_id(&mut self, channel_id: ::models::EncodedHash) {
    self.channel_id = channel_id;
  }

  pub fn with_channel_id(mut self, channel_id: ::models::EncodedHash) -> ChannelCloseSoloTx {
    self.channel_id = channel_id;
    self
  }

  pub fn channel_id(&self) -> &::models::EncodedHash {
    &self.channel_id
  }


  pub fn set_from_id(&mut self, from_id: ::models::EncodedHash) {
    self.from_id = from_id;
  }

  pub fn with_from_id(mut self, from_id: ::models::EncodedHash) -> ChannelCloseSoloTx {
    self.from_id = from_id;
    self
  }

  pub fn from_id(&self) -> &::models::EncodedHash {
    &self.from_id
  }


  pub fn set_payload(&mut self, payload: String) {
    self.payload = payload;
  }

  pub fn with_payload(mut self, payload: String) -> ChannelCloseSoloTx {
    self.payload = payload;
    self
  }

  pub fn payload(&self) -> &String {
    &self.payload
  }


  pub fn set_ttl(&mut self, ttl: i64) {
    self.ttl = Some(ttl);
  }

  pub fn with_ttl(mut self, ttl: i64) -> ChannelCloseSoloTx {
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

  pub fn with_fee(mut self, fee: i64) -> ChannelCloseSoloTx {
    self.fee = fee;
    self
  }

  pub fn fee(&self) -> &i64 {
    &self.fee
  }


  pub fn set_nonce(&mut self, nonce: i64) {
    self.nonce = Some(nonce);
  }

  pub fn with_nonce(mut self, nonce: i64) -> ChannelCloseSoloTx {
    self.nonce = Some(nonce);
    self
  }

  pub fn nonce(&self) -> Option<&i64> {
    self.nonce.as_ref()
  }

  pub fn reset_nonce(&mut self) {
    self.nonce = None;
  }

  pub fn set_poi(&mut self, poi: String) {
    self.poi = poi;
  }

  pub fn with_poi(mut self, poi: String) -> ChannelCloseSoloTx {
    self.poi = poi;
    self
  }

  pub fn poi(&self) -> &String {
    &self.poi
  }


}


