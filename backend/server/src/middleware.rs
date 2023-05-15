use actix_web::{dev::ServiceRequest, HttpResponse, Result};
use std::net::Ipv4Addr;

// TODO: use with app
/// Allows only private IPs
pub(crate) fn local_network_only(req: &ServiceRequest) -> Result<HttpResponse> {
    let request_connection_info = req.connection_info();
    let source_address = request_connection_info.peer_addr();
    if source_address.is_none() {
        return Ok(HttpResponse::Forbidden().finish());
    }
    // TODO: support also IPv6 when functions like `is_global` are stable
    let source_address = source_address.unwrap().parse::<Ipv4Addr>();
    if source_address.is_err() {
        return Ok(HttpResponse::Forbidden().finish());
    }

    if !source_address.unwrap().is_link_local() {
        return Ok(HttpResponse::Forbidden().finish());
    }

    Ok(HttpResponse::Ok().finish())
}
