use reqwest::Method;

use crate::api::req::HttpReq;
use crate::api::resp::RespType;
use crate::api::TGReq;
use crate::errors::TGBotResult;
use crate::types::{ChatRef, MessageId, ReplyMarkup, ToChatRef, ToMessageId, ToSourceChat};
use crate::vision::PossibilityMessage;

/// Use this method to edit only the reply markup of messages sent by the bot.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct EditMessageReplyMarkup {
  chat_id: ChatRef,
  message_id: MessageId,
  #[serde(skip_serializing_if = "Option::is_none")]
  reply_markup: Option<ReplyMarkup>,
}

impl TGReq for EditMessageReplyMarkup {
  type Resp = RespType<PossibilityMessage>;

  fn request(&self) -> TGBotResult<HttpReq> {
    HttpReq::json_req(Method::POST, "editMessageCaption", self)
  }
}

impl EditMessageReplyMarkup {
  pub fn new<C, M, R>(chat: C, message_id: M, reply_markup: Option<R>) -> Self
    where C: ToChatRef, M: ToMessageId, R: Into<ReplyMarkup> {
    EditMessageReplyMarkup {
      chat_id: chat.to_chat_ref(),
      message_id: message_id.to_message_id(),
      reply_markup: reply_markup.map(|r| r.into()),
    }
  }
}

/// Edit reply markup of messages sent by the bot.
pub trait CanEditMessageReplyMarkup {
  fn edit_reply_markup<R>(&self, reply_markup: Option<R>) -> EditMessageReplyMarkup where R: Into<ReplyMarkup>;
}

impl<M> CanEditMessageReplyMarkup for M where M: ToMessageId + ToSourceChat {
  fn edit_reply_markup<R>(&self, reply_markup: Option<R>) -> EditMessageReplyMarkup where R: Into<ReplyMarkup> {
    EditMessageReplyMarkup::new(self.to_source_chat(), self.to_message_id(), reply_markup)
  }
}
