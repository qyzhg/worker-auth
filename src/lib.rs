mod svc;
mod models;
mod utils;
mod handlers;

use worker::*;
use utils::return_response;

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    // 设置panic hook
    console_error_panic_hook::set_once();
    // 创建路由
    let router = Router::new();
    // 接口定义
    router
        // 用户注册
        .post_async("/api/user/register", handlers::api::user::register_handler)
        .run(req, env)
        .await
}

