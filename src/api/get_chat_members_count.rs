use reqwest::Method;

use crate::api::req::HttpReq;
use crate::api::resp::RespType;
use crate::api::TGReq;
use crate::errors::TGBotResult;
use crate::types::*;

/// Use this method to get the number of members in a chat.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct GetChatMembersCount {
  chat_id: ChatRef
}


impl TGReq for GetChatMembersCount {
  type Resp = RespType<i64>;

  fn request(&self) -> TGBotResult<HttpReq> {
    HttpReq::json_req(Method::POST, "getChatMembersCount", self)
  }
}

impl GetChatMembersCount {
  pub fn new<C>(chat: C) -> Self where C: ToChatRef {
    GetChatMembersCount {
      chat_id: chat.to_chat_ref()
    }
  }
}

/// Get the number of members in a chat.
pub trait CanGetChatMembersCount {
  fn get_members_count(&self) -> GetChatMembersCount;
}

impl<C> CanGetChatMembersCount for C where C: ToChatRef {
  fn get_members_count(&self) -> GetChatMembersCount {
    GetChatMembersCount::new(self)
  }
}
