use anyhow::Error;
use serde_json::Value;
use worker::RouteContext;

pub fn register(body: Value, ctx: RouteContext<()>) -> Result<String, Error> {
    // 从上下文中获取d1数据库实例
    let db = match ctx.env.d1("DB"){
        Ok(db) => db,
        Err(e) => return Err(Error::from(e)),
    };
    // 将body解析为用户结构体

    Ok("新用户注册成功".to_string())
}
