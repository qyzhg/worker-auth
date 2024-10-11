use anyhow::Error;
use worker::{console_log, RouteContext};
use crate::{models, utils};
use crate::models::status_code::StatusCode;
use argon2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Argon2,
};
use serde_json::{json, Value};

use jwt_compact::{prelude::*, alg::{Hs256, Hs256Key}};
use chrono::{Duration, Utc};


pub(crate) async fn login(user: models::user::User, ctx: RouteContext<()>) -> Result<(String, Option<Value>), (StatusCode, Error)>{
    // 从上下文中获取d1数据库实例
    let db = match ctx.env.d1("DB") {
        Ok(db) => db,
        Err(e) => return Err((StatusCode::InternalServerError, Error::from(e))),
    };
    // 密码base64解码
    let password = match utils::b64::base64_decode(user.password.clone()) {
        Ok(password) => password,
        Err(e) => return Err((StatusCode::InternalServerError, Error::from(e)))
    };
    console_log!("密码base64解码成功::{}", password);
    // 数据库中查询用户
    let statement = db
        .prepare(
            r#"SELECT * FROM users WHERE name = ? AND is_deleted IS NULL"#);
    let query = match statement.bind(&[user.name.clone().into()]){
        Ok(q) => q,
        Err(e) => return Err((StatusCode::InternalServerError, Error::from(e)))
    };
    let option_user = match query.first::<models::user::User>(None).await{
        Ok(result) => result,
        Err(e) => return Err((StatusCode::InternalServerError, Error::from(e)))
    };
    let db_user = match option_user{
        Some(user) => user,
        None => return Err((StatusCode::Unauthorized, Error::from(anyhow::anyhow!("用户不存在"))))
    };
    console_log!("用户查询成功::{:?}", db_user);
    // 密码校验
    let argon2 = Argon2::default();
    // 将数据库中的密码解哈希
    let pwd_to_verify = db_user.password.clone();
    let parsed_hash = match PasswordHash::new(&pwd_to_verify){
        Ok(hash) => hash,
        Err(e) => return Err((StatusCode::InternalServerError, Error::msg(e.to_string())))
    };
    // 校验
    if !argon2.verify_password(password.as_ref(), &parsed_hash).is_ok(){
        return Err((StatusCode::Unauthorized, Error::from(anyhow::anyhow!("用户名/密码错误"))))
    };
    // 生成jwt
    let claims = models::claims::Claims{
        sub: db_user.id.unwrap().to_string(),
        group: "user".to_string(),
        exp: 0,
    };
    let time_options = TimeOptions::default();
    let jwt_secret = match ctx.env.var("jwt-secret"){
        Ok(secret) => secret.to_string(),
        Err(e) => return Err((StatusCode::InternalServerError, Error::from(e)))
    };
    console_log!("jwt-secret::{}", jwt_secret);
    let key = Hs256Key::new(jwt_secret);
    let header = Header::empty().with_key_id("1");
    let claims = Claims::new(claims)
        .set_duration_and_issuance(&time_options, Duration::hours(72))
        .set_not_before(Utc::now());
    let token_string = match Hs256.token(&header, &claims, &key){
        Ok(token) => token,
        Err(e) => return Err((StatusCode::InternalServerError, Error::msg(e.to_string())))
    };
    console_log!("生成jwt成功::{}", token_string);
    Ok(("登录成功".to_string(), Some(json!({"jwt": token_string}))))
}
