use crate::models::status_code::StatusCode;
use crate::{models, svc};
use crate::utils::return_response;
use worker::{console_error, console_log, Request, Response, RouteContext};

pub(crate) async fn register_handler(
    mut req: Request,
    ctx: RouteContext<()>,
) -> worker::Result<Response> {
    // 解析body参数
    let user = match req.json::<models::user::User>().await {
        Ok(body) => body,
        Err(e) => {
            return return_response::err(
                StatusCode::InternalServerError,
                Some(format!("参数解析失败::{:?}", e)),
                None,
            );
        }
    };
    // 调用svc中的注册方法
    return match svc::api::user::register::register(user, ctx).await {
        Ok(msg) => return_response::ok(Some(msg), None),
        Err(e) => return_response::err(e.0, Some(format!("注册失败::{:?}", e.1)), None),
    };
}

pub(crate) async fn login_handler(
    mut req: Request,
    ctx: RouteContext<()>,
) -> worker::Result<Response> {
    // 解析body参数
    let user = match req.json::<models::user::User>().await {
        Ok(body) => body,
        Err(e) => {
            return return_response::err(
                StatusCode::InternalServerError,
                Some(format!("参数解析失败::{:?}", e)),
                None,
            );
        }
    };
    console_log!("req:{:?}", req);
    return match svc::api::user::login::login(user, ctx).await{
        Ok(r) => return_response::ok(Some(r.0), r.1),
        Err(e) => return_response::err(e.0, Some(format!("登录失败::{:?}", e.1)), None),
    }
}