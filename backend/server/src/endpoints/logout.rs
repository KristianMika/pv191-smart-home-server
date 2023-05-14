use actix_web::{post, HttpResponse};

use crate::endpoints::auth::create_logout_response;

#[post("/logout")]
pub(crate) async fn post_logout() -> HttpResponse {
    create_logout_response()
}
