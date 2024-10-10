use worker::{Request, Response, RouteContext};
use serde_json::Value;
use crate::svc;
use crate::utils::return_response;

pub async fn register_handler(
    mut req: Request,
    ctx: RouteContext<()>,
) -> worker::Result<Response> {
    let body = match req.json::<Value>().await {
        Ok(body) => body,
        Err(e) => {
            return return_response::from_json(500, Some(format!("参数解析失败::{:?}", e)), None);
        }
    };
    match svc::api::user::register::register(body, ctx) {
        Ok(msg) => return_response::from_json(200, Some(msg), None),
        Err(e) => return_response::from_json(500, Some(format!("注册失败::{:?}", e)), None),
    }
}