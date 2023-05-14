use actix_jwt_auth_middleware::FromRequest;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, FromRequest, Default)]
pub(crate) struct UserClaims {
    id: u64,
}

impl UserClaims {
    pub fn new(id: u64) -> Self {
        Self { id }
    }

    pub fn get_id(&self) -> u64 {
        self.id
    }
}
