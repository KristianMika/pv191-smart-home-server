use actix_jwt_auth_middleware::{AuthError, TokenSigner};
use actix_web::{cookie::Cookie, web, HttpResponse};
use jwt_compact::alg::Ed25519;

use crate::auth::User;

/// Creates a jwt indicator cookie that is not-HttpOnly
///
/// It's main purpose is to indicate to front-end when
/// the user is loged-in, as we can't access httpOnly
/// cookies from JS
fn create_jwt_indicator_cookie() -> Cookie<'static> {
    Cookie::new("jwt_set", "dummy value")
}

/// Creates a response with access, refresh, and dummy
/// jwt indicator cookies
pub(crate) fn create_auth_response(
    user: User,
    token_signer: web::Data<TokenSigner<User, Ed25519>>,
) -> Result<HttpResponse, AuthError> {
    let mut access_cookie = token_signer.create_access_cookie(&user)?;
    let mut refresh_cookie = token_signer.create_refresh_cookie(&user)?;
    let dummy_jwt_indicator_cookie = create_jwt_indicator_cookie();
    access_cookie.set_http_only(true);
    refresh_cookie.set_http_only(true);

    Ok(HttpResponse::Ok()
        .cookie(access_cookie)
        .cookie(refresh_cookie)
        .cookie(dummy_jwt_indicator_cookie)
        .finish())
}
