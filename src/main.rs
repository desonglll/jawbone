use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use jawbone::music::stream_music;
use jawbone::routes::{audio_list, index};
use std::net::{Ipv4Addr, SocketAddrV4};
use tracing_actix_web::TracingLogger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug"); // 设置日志级别
    env_logger::init(); // 初始化日志记录
    let host = Ipv4Addr::new(0, 0, 0, 0);
    HttpServer::new(move || {
        let cors = Cors::default().allow_any_origin();
        App::new()
            .wrap(Logger::default()) // 启用 Logger 中间件
            .wrap(cors)
            .route("/", web::get().to(index))
            .route("/stream/{path}", web::get().to(stream_music))
            .route("/audio_list", web::get().to(audio_list))
    })
    .bind(SocketAddrV4::new(host, 8888))? // 绑定本地端口
    .run()
    .await
}
