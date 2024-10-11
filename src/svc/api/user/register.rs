use crate::models::status_code::StatusCode;
use crate::{models, utils};
use anyhow::Error;
use argon2::{
    password_hash::{PasswordHasher, SaltString},
    Argon2,
};
use base64::{engine::general_purpose, Engine as _};
use rand_core::OsRng;
use worker::{console_error, console_log, RouteContext};

pub(crate) async fn register(user: models::user::User, ctx: RouteContext<()>) -> Result<String, (StatusCode, Error)> {
    // 从上下文中获取d1数据库实例
    let db = match ctx.env.d1("DB") {
        Ok(db) => db,
        Err(e) => return Err((StatusCode::InternalServerError, Error::from(e))),
    };
    // 从上下文中获取kv数据库实例
    // let kv = match ctx.env.kv("kv_auth") {
    //     Ok(kv) => kv,
    //     Err(e) => return Err((StatusCode::InternalServerError, Error::from(e))),
    // };
    // todo: 校验验证码是否正确
    // let kv_captcha =  match kv.get(&*user.email).text().await{
    //     Ok(Some(value)) => value,
    //     Ok(None) => {
    //         return Err((StatusCode::BadRequest, Error::msg("验证码不存在")))
    //     }
    //     Err(e) => {
    //         return Err((
    //             StatusCode::InternalServerError,
    //             Error::msg(format!("kv获取验证码失败::{:?}", e)),
    //         ))
    //     }
    // };
    // console_log!("kv验证码读取成功！::{}", kv_captcha);
    // 密码base64解码
    let password = match utils::b64::base64_decode(user.password) {
        Ok(password) => password,
        Err(e) => return Err((StatusCode::BadRequest, Error::msg(e))),
    };
    console_log!("密码base64解码成功::{}", password);
    // 密码格式校验
    if let Err(e) = utils::validate::validate_password(&password) {
        return Err((StatusCode::BadRequest, e));
    }
    console_log!("密码格式校验通过！");
    // 邮箱格式校验
    if !utils::validate::validate_email(&user.email) {
        return Err((StatusCode::BadRequest, Error::msg("邮箱格式不正确")));
    }
    console_log!("邮箱格式校验通过！");
    // 检查用户名是否存在
    match db
        .prepare(r#"SELECT id FROM users WHERE name = ?"#)
        .bind(&[user.name.clone().into()])
    {
        Ok(pre) => {
            let result = pre.first::<usize>(Some("id")).await;
            match result {
                Ok(row) => {
                    console_log!("用户名查询成功");
                    if let Some(id) = row {
                        console_error!("用户名已存在！id:{}", id);
                        return Err((StatusCode::BadRequest, Error::msg("用户名已存在!")));
                    }
                }
                Err(e) => {
                    console_error!("用户名查询失败::{}", e);
                    return Err((
                        StatusCode::InternalServerError,
                        Error::msg(format!("用户名查询失败::{:?}", e.to_string())),
                    ));
                }
            }
        }
        Err(e) => {
            console_error!("数据库name查询失败::{}", e);
            return Err((StatusCode::InternalServerError, Error::from(e)));
        }
    };
    console_log!("用户名冲突校验通过！");
    // 检查邮箱是否存在
    match db
        .prepare(r#"SELECT id FROM users WHERE email = ?"#)
        .bind(&[user.email.clone().into()])
    {
        Ok(pre) => {
            let result = pre.first::<usize>(Some("id")).await;
            match result {
                Ok(row) => {
                    console_log!("邮箱查询成功");
                    if let Some(id) = row {
                        console_error!("邮箱已存在！id: {}", id);
                        return Err((StatusCode::BadRequest, Error::msg("邮箱已存在!")));
                    }
                }
                Err(e) => {
                    console_error!("邮箱查询失败::{}", e);
                    return Err((
                        StatusCode::InternalServerError,
                        Error::msg(format!("邮箱查询失败::{:?}", e.to_string())),
                    ));
                }
            }
        }
        Err(e) => {
            console_error!("数据库email查询失败::{}", e);
            return Err((StatusCode::InternalServerError, Error::from(e)));
        }
    }
    console_log!("邮箱冲突校验通过！");
    // 创建用户

    // 生成一个随机盐
    let salt = SaltString::generate(&mut OsRng);
    // 创建 Argon2 默认实例，使用 Argon2id v1.3 算法
    let argon2 = Argon2::default();
    // 将密码和盐一起哈希，生成符合 PHC 规范的字符串 ($argon2id$v=19$...)
    let password_hash = match argon2.hash_password(password.as_ref(), &salt) {
        Ok(hash) => hash.to_string(),
        Err(e) => return Err((StatusCode::InternalServerError, Error::msg(e.to_string()))),
    };
    console_log!("密码哈希成功: {}", password_hash);
    // 落库
    match db
        .prepare(r#"INSERT INTO users (name, email, password) VALUES (?, ?, ?);"#)
        .bind(&[
            user.name.clone().into(),
            user.email.clone().into(),
            password_hash.clone().into(),
        ]) {
        Ok(pre) => match pre.run().await {
            Ok(_) => {
                console_log!("插入用户表成功");
            }
            Err(e) => {
                console_error!("db error: {:?}", e);
                return Err((StatusCode::InternalServerError, Error::from(e)));
            }
        },
        Err(e) => return Err((StatusCode::InternalServerError, Error::from(e))),
    }
    Ok(format!("新用户[{}]创建成功", user.name))
}
