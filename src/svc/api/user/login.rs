use anyhow::Error;
use base64::engine::general_purpose;
use worker::{console_log, RouteContext};
use crate::{models, utils};
use crate::models::status_code::StatusCode;

pub(crate) async fn login(user: models::user::User, ctx: RouteContext<()>) -> Result<String, (StatusCode, Error)>{
    // 从上下文中获取d1数据库实例
    let db = match ctx.env.d1("DB") {
        Ok(db) => db,
        Err(e) => return Err((StatusCode::InternalServerError, Error::from(e))),
    };
    // 密码base64解码
    let password = match utils::b64::base64_decode(user.password) {
        Ok(password) => password,
        Err(e) => return Err((StatusCode::InternalServerError, Error::from(e)))
    };
    console_log!("密码base64解码成功::{}", password);
    Ok("登录成功".to_string())
}