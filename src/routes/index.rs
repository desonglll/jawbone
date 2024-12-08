use actix_web::{HttpRequest, HttpResponse, Responder};

pub async fn index(req: HttpRequest) -> impl Responder {
    let connection_info = req.connection_info();
    let host = connection_info.host();

    HttpResponse::Ok().body(format!("Hello from {}", host))
}
