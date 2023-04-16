use crate::{
    auth::User,
    endpoints::models::{LoginRequest, Response},
    server_repo::ServerRepo,
    state::ServerState,
};
use actix_jwt_auth_middleware::{AuthResult, TokenSigner};
use actix_web::{post, web, HttpResponse};
use jwt_compact::alg::Ed25519;
use log::error;

#[post("/login")]
pub(crate) async fn post_login(
    login_request: web::Json<LoginRequest>,
    state: web::Data<ServerState>,
    token_signer: web::Data<TokenSigner<User, Ed25519>>,
) -> AuthResult<HttpResponse> {
    let user = match state.repo.get_user(&login_request.login) {
        Ok(val) => val,
        Err(err) => {
            error!(
                "Couldn't get user with login {}: {}",
                &login_request.login, err
            );
            return Ok(HttpResponse::InternalServerError().finish());
        }
    };

    if user.is_none() {
        return Ok(HttpResponse::BadRequest().json(Response {
            message: "Invalid username or password".into(),
        }));
    }
    let user = user.unwrap();
    let verification_result = bcrypt::verify(&login_request.password, &user.user_password_hash);
    let was_verification_successfull = match verification_result {
        Err(err) => {
            error!("An error occured while verifying password: {}", err);
            return Ok(HttpResponse::InternalServerError().finish());
        }
        Ok(val) => val,
    };
    if !was_verification_successfull {
        return Ok(HttpResponse::BadRequest().json(Response {
            message: "Invalid username or password".into(),
        }));
    }
    let user = User::default();
    Ok(HttpResponse::Ok()
        .cookie(token_signer.create_access_cookie(&user)?)
        .cookie(token_signer.create_refresh_cookie(&user)?)
        .finish())
}
