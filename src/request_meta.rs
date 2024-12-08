use actix_web::HttpRequest;
use std::net::Ipv4Addr;
use std::str::FromStr;

pub struct ServerHost {
    pub host: Ipv4Addr,
    pub port: u16,
}
pub fn get_host(req: HttpRequest) -> ServerHost {
    let connection_info = req.connection_info();
    let host = connection_info
        .host()
        .split(':')
        .nth(0)
        .unwrap_or("0.0.0.0");
    let host = host.parse::<Ipv4Addr>().unwrap_or(Ipv4Addr::UNSPECIFIED);
    let port = connection_info
        .host()
        .split(':')
        .nth(1)
        .unwrap_or("443")
        .parse::<u16>()
        .unwrap();
    ServerHost { host, port }
}
