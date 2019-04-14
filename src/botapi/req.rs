use std::sync::Arc;

use crate::{Config, TGFuture};
use crate::botapi::resp::{HttpResp, TGResp};

pub trait TGReq {
  type Resp: TGResp + 'static;

  fn request(&self, cfg: Arc<Config>) -> TGFuture<HttpResp>;
}

impl<'a, Req: TGReq> TGReq for &'a Req {
  type Resp = Req::Resp;

  fn request(&self, cfg: Arc<Config>) -> TGFuture<HttpResp> {
    (*self).request(cfg)
  }
}

impl<'a, Req: TGReq> TGReq for &'a mut Req {
  type Resp = Req::Resp;

  fn request(&self, cfg: Arc<Config>) -> TGFuture<HttpResp> {
    (**self).request(cfg)
  }
}
