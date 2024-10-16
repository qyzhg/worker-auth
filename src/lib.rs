mod handlers;
mod models;
mod svc;
mod utils;

use worker::*;

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    // 设置panic hook
    console_error_panic_hook::set_once();
    // 创建路由
    let router = Router::new();
    // 接口定义
    router
        .get_async("/", |_req, _ctx| async move {
            Response::ok("来了老弟")
        })
        // 用户注册
        .post_async("/user/register", handlers::api::user::register_handler)
        // 用户登录
        .post_async("/user/login", handlers::api::user::login_handler)
        // 用户组管理
        .run(req, env)
        .await
}
