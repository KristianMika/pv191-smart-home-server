use crate::{
    endpoints::models::{LoginRequest, Response},
    server_repo::ServerRepo,
    state::ServerState,
};
use actix_web::{post, web, HttpResponse, Responder};
use log::error;

#[post("/login")]
pub(crate) async fn post_login(
    login_request: web::Json<LoginRequest>,
    state: web::Data<ServerState>,
) -> impl Responder {
    let user = match state.repo.get_user(&login_request.login) {
        Ok(val) => val,
        Err(err) => {
            error!(
                "Couldn't get user with login {}: {}",
                &login_request.login, err
            );
            return HttpResponse::InternalServerError().finish();
        }
    };

    if user.is_none() {
        return HttpResponse::BadRequest().json(Response {
            message: "Invalid username or password".into(),
        });
    }
    let user = user.unwrap();
    let verification_result = bcrypt::verify(&login_request.password, &user.user_password_hash);
    let was_verification_successfull = match verification_result {
        Err(err) => {
            error!("An error occured while verifying password: {}", err);
            return HttpResponse::InternalServerError().finish();
        }
        Ok(val) => val,
    };
    if !was_verification_successfull {
        return HttpResponse::BadRequest().json(Response {
            message: "Invalid username or password".into(),
        });
    }

    set_jwt_cookie();
    HttpResponse::Ok().finish()
}

fn set_jwt_cookie() {
    todo!()
}
