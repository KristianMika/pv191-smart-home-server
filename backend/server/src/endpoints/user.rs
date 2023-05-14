use crate::{endpoints::models::UserResponse, models::UserClaims, state::ServerState};
use actix_web::{get, web, HttpResponse};
use common::server_repo::ServerRepo;
use log::error;

#[get("/user")]
pub(crate) async fn get_user(
    state: web::Data<ServerState>,
    user_claims: UserClaims,
) -> HttpResponse {
    let Ok(Some(user)) = state.repo.get_user_by_id(user_claims.get_id() as i32) else {
        error!("Couldn't get user with ID {}", user_claims.get_id());
        return HttpResponse::BadRequest().finish();
    };

    let response = UserResponse::new(user.first_name);
    HttpResponse::Ok().json(response)
}
