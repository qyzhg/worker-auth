use crate::models;
use models::status_code::StatusCode;
use serde_json::{json, to_value, Value};
use worker::{Response, Result};

fn from_json(resp: models::resp::Response) -> Result<Response> {
    return match to_value(resp) {
        Ok(json_value) => Response::from_json(&json_value),
        Err(e) => {
            Response::from_json(&json!({"code": 500, "message": format!("json解析错误: {}", e)}))
        }
    };
}
pub(crate) fn ok(message: Option<String>, data: Option<Value>) -> Result<Response> {
    let resp = models::resp::Response {
        code: StatusCode::Ok as u16,
        message,
        data,
    };
    from_json(resp)
}

pub(crate) fn err(
    code: StatusCode,
    message: Option<String>,
    data: Option<Value>,
) -> Result<Response> {
    let resp = models::resp::Response {
        code: code as u16,
        message,
        data,
    };
    from_json(resp)
}
