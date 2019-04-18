use std::ops::Not;

use reqwest::Method;

use crate::api::req::HttpReq;
use crate::api::resp::{JsonTrueToUnitResp, RespType};
use crate::api::TGReq;
use crate::errors::TGBotResult;
use crate::types::*;

/// Use this method to pin a message in a supergroup or a channel.
/// The bot must be an administrator in the chat for this to work
/// and must have the ‘can_pin_messages’ admin right in the supergroup
/// or ‘can_edit_messages’ admin right in the channel.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct PinChatMessage {
  chat_id: ChatRef,
  message_id: MessageId,
  #[serde(skip_serializing_if = "Not::not")]
  disable_notification: bool,
}


impl TGReq for PinChatMessage {
  type Resp = JsonTrueToUnitResp;

  fn request(&self) -> TGBotResult<HttpReq> {
    HttpReq::json_req(Method::POST, "pinChatMessage", self)
  }
}


impl PinChatMessage {
  fn new<C, M>(chat: C, message: M) -> Self where C: ToChatRef, M: ToMessageId {
    Self {
      chat_id: chat.to_chat_ref(),
      message_id: message.to_message_id(),
      disable_notification: false,
    }
  }

  pub fn disable_notification(&mut self) -> &mut Self {
    self.disable_notification = true;
    self
  }
}

pub trait CanPinMessage {
  fn pin(&self) -> PinChatMessage;
}

impl<M> CanPinMessage for M where M: ToMessageId + ToSourceChat {
  fn pin(&self) -> PinChatMessage {
    PinChatMessage::new(self.to_source_chat(), self.to_message_id())
  }
}
