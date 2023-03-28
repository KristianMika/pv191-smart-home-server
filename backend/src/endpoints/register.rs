use crate::{
    endpoints::models::RegisterRequest,
    server_repo::{DbError, ServerRepo},
    state::ServerState,
};
use actix_web::{post, web, HttpResponse, Responder};
use bcrypt::hash;
use log::error;

static BCRYPT_COST: u32 = 10;

#[post("/register")]
pub(crate) async fn post_register(
    request: web::Json<RegisterRequest>,
    state: web::Data<ServerState>,
) -> impl Responder {
    if !request.is_valid() {
        return HttpResponse::BadRequest().finish();
    }
    let hashed_password = match hash(&request.password, BCRYPT_COST) {
        Ok(val) => val,
        Err(err) => {
            error!("Couldn't hash password: {}", err);
            return HttpResponse::InternalServerError().finish();
        }
    };

    let created_user =
        state
            .repo
            .create_new_user(&request.first_name, &request.login, &hashed_password);
    if let Err(err) = created_user {
        match err.current_context() {
            DbError::ConstraintError => return HttpResponse::BadRequest().finish(),
            _ => {
                error!("{:?}", err);
                return HttpResponse::InternalServerError().finish();
            }
        }
    }
    HttpResponse::Ok().finish()
}
