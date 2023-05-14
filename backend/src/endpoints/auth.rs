use actix_jwt_auth_middleware::{AuthError, TokenSigner};
use actix_web::{cookie::Cookie, web, HttpResponse};
use jwt_compact::alg::Ed25519;

use crate::auth::UserClaims;

pub(crate) const ACCESS_TOKEN_COOKIE_NAME: &str = "access_token";
pub(crate) const REFRESH_TOKEN_COOKIE_NAME: &str = "refresh_token";
pub(crate) const JWT_INDICATOR_COOKIE_NAME: &str = "jwt_set";

/// Creates a jwt indicator cookie that is not-HttpOnly
///
/// It's main purpose is to indicate to front-end when
/// the user is loged-in, as we can't access httpOnly
/// cookies from JS
fn create_jwt_indicator_cookie() -> Cookie<'static> {
    Cookie::new(JWT_INDICATOR_COOKIE_NAME, "dummy value")
}

/// Creates a response with access, refresh, and dummy
/// jwt indicator cookies
pub(crate) fn create_auth_response(
    user: UserClaims,
    token_signer: web::Data<TokenSigner<UserClaims, Ed25519>>,
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

pub(crate) fn create_logout_response() -> HttpResponse {
    let mut access_token_removal_cookie = Cookie::new(ACCESS_TOKEN_COOKIE_NAME, "");
    let mut refresh_token_removal_cookie = Cookie::new(REFRESH_TOKEN_COOKIE_NAME, "");
    let mut jwt_indicator_cookie = create_jwt_indicator_cookie();

    access_token_removal_cookie.make_removal();
    refresh_token_removal_cookie.make_removal();
    jwt_indicator_cookie.make_removal();

    HttpResponse::Ok()
        .cookie(access_token_removal_cookie)
        .cookie(refresh_token_removal_cookie)
        .cookie(jwt_indicator_cookie)
        .finish()
}
