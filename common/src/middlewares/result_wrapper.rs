use crate::response::Res;
use sea_orm::prelude::async_trait;
use serde_json::Value;
use silent::prelude::ResBody;
use silent::{Handler, MiddleWareHandler, Next, Request, Response, Result};

#[derive(Default)]
pub struct ResultWrapper;

#[allow(dead_code)]
impl ResultWrapper {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl MiddleWareHandler for ResultWrapper {
    async fn handle(&self, req: Request, next: &Next) -> Result<Response> {
        let mut res = next.call(req).await?;
        if res.headers().get("content-type").unwrap() == "application/json" {
            if let ResBody::Once(body) = &res.body() {
                let data = String::from_utf8_lossy(body.to_vec().as_slice()).to_string();
                let result_data = serde_json::from_slice::<Value>(body.to_vec().as_slice());
                if let Ok(data) = result_data {
                    res.copy_from_response(Res::with_data(data).into());
                } else {
                    res.copy_from_response(Res::with_data(data).into());
                }
            }
        }
        Ok(res)
    }
}
