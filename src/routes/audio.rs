use crate::music::audio::list;
use crate::request_meta::get_host;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use std::collections::HashMap;

pub async fn audio_list(req: HttpRequest) -> impl Responder {
    let query_string = req.query_string();
    let query: web::Query<HashMap<String, String>> = web::Query::from_query(query_string).unwrap();

    let home_dir = dirs::home_dir().expect("Failed to get home directory");
    let default_dir = home_dir.join("Music");
    let dir = query
        .get("dir")
        .map(String::to_string)
        .unwrap_or(default_dir.to_str().unwrap().to_string());

    let server = get_host(req);

    let mut audio_list = list(&dir);
    let mut encoded_audio_list = audio_list.encode();

    encoded_audio_list
        .as_mut()
        .set_stream(server.host, server.port, "stream");

    HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .body(serde_json::to_string(&encoded_audio_list).unwrap())
}
