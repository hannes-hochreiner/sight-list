use crate::objects::contexts::ErrorContext;
use rocket::{
    response::{self, Responder},
    Request,
};
use rocket_dyn_templates::Template;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SightListError {
    #[error("update error")]
    UpdateError,
    #[error("error summing latitudes or longitudes")]
    SumError,
    #[error("error querying database")]
    RocketSqlxError(#[from] rocket_db_pools::sqlx::Error),
}

#[rocket::async_trait]
impl<'r> Responder<'r, 'static> for SightListError {
    fn respond_to(self, request: &'r Request<'_>) -> response::Result<'static> {
        let msg = format!("{:?}", self);

        Template::render("error", ErrorContext { message: msg })
            .respond_to(request)
            .and_then(|mut resp| {
                resp.set_raw_header("HX-Retarget", "#error");
                Ok(resp)
            })
    }
}
