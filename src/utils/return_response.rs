use serde_json::{json, to_value, Value};
use worker::{Result, Response};
use crate::models;

pub(crate) fn from_json(code: u32, message: Option<String>, data: Option<Value>) -> Result<Response> {
    let resp = models::resp::Response {
        code,
        message,
        data,
    };
   return match to_value(resp){
        Ok(json_value) => {
            Response::from_json(&json_value)
        }
        Err(e) => {
            Response::from_json(&json!({"code": 500, "message": format!("json解析错误: {}", e)}))
        }
    };
}