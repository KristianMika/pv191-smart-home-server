use actix_jwt_auth_middleware::{AuthResult, TokenSigner};
use actix_web::{post, web, HttpResponse};
use bcrypt::hash;
use common::server_repo::{DbError, ServerRepo};
use jwt_compact::alg::Ed25519;
use log::error;

use crate::{
    endpoints::{
        auth::create_auth_response,
        models::{RegisterRequest, Response},
    },
    models::UserClaims,
    state::ServerState,
};

static BCRYPT_COST: u32 = 10;

#[post("/register")]
pub(crate) async fn post_register(
    mut request: web::Json<RegisterRequest>,
    state: web::Data<ServerState>,
    token_signer: web::Data<TokenSigner<UserClaims, Ed25519>>,
) -> AuthResult<HttpResponse> {
    request.trim_inputs();
    if !request.is_valid() {
        return Ok(HttpResponse::BadRequest().json(Response {
            message: "Invalid request".into(),
        }));
    }
    let hashed_password = match hash(&request.password, BCRYPT_COST) {
        Ok(val) => val,
        Err(err) => {
            error!("Couldn't hash password: {}", err);
            return Ok(HttpResponse::InternalServerError().finish());
        }
    };

    let created_user =
        state
            .repo
            .create_new_user(&request.first_name, &request.login, &hashed_password);
    if let Err(err) = created_user {
        match err.current_context() {
            DbError::ConstraintError => {
                return Ok(HttpResponse::BadRequest().json(Response {
                    message: "The username already exists".into(),
                }))
            }
            _ => {
                error!("{:?}", err);
                return Ok(HttpResponse::InternalServerError().finish());
            }
        }
    }
    let user = created_user.unwrap();

    create_auth_response(UserClaims::new(user.id as u64), token_signer)
}
