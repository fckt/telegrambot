use std::ops::Not;
use std::borrow::Cow;
use crate::types::{ChatRef, MessageId, ParseMode, ReplyMarkup, ToChatRef, ToMessageId, ToSourceChat};
use crate::api::TGReq;
use crate::api::resp::RespType;
use crate::vision::PossibilityMessage;
use crate::errors::TGBotResult;
use crate::api::req::HttpReq;
use reqwest::Method;

/// Use this method to edit text messages sent by the bot.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct EditMessageText<'s> {
  chat_id: ChatRef,
  message_id: MessageId,
  text: Cow<'s, str>,
  #[serde(skip_serializing_if = "Option::is_none")]
  parse_mode: Option<ParseMode>,
  #[serde(skip_serializing_if = "Not::not")]
  disable_web_page_preview: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  reply_markup: Option<ReplyMarkup>,
}


impl<'s> TGReq for EditMessageText<'s> {
  type Resp = RespType<PossibilityMessage>;

  fn request(&self) -> TGBotResult<HttpReq> {
    HttpReq::json_req(Method::POST, "editMessageText", self)
  }
}


impl<'s> EditMessageText<'s> {
  pub fn new<C, M, T>(chat: C, message_id: M, text: T) -> Self
    where C: ToChatRef, M: ToMessageId, T: Into<Cow<'s, str>> {
    EditMessageText {
      chat_id: chat.to_chat_ref(),
      message_id: message_id.to_message_id(),
      text: text.into(),
      parse_mode: None,
      disable_web_page_preview: false,
      reply_markup: None,
    }
  }

  pub fn parse_mode(&mut self, parse_mode: ParseMode) -> &mut Self {
    self.parse_mode = Some(parse_mode);
    self
  }

  pub fn disable_preview(&mut self) -> &mut Self {
    self.disable_web_page_preview = true;
    self
  }

  pub fn reply_markup<R>(&mut self, reply_markup: R) -> &mut Self where R: Into<ReplyMarkup> {
    self.reply_markup = Some(reply_markup.into());
    self
  }
}

/// Edit text of messages sent by the bot.
pub trait CanEditMessageText {
  fn edit_text<'s, T>(&self, text: T) -> EditMessageText<'s> where T: Into<Cow<'s, str>>;
}

impl<M> CanEditMessageText for M where M: ToMessageId + ToSourceChat {
  fn edit_text<'s, T>(&self, text: T) -> EditMessageText<'s> where T: Into<Cow<'s, str>> {
    EditMessageText::new(self.to_source_chat(), self.to_message_id(), text)
  }
}
