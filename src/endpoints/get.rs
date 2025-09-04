
use crate::{sql_service::fetches, AppState};
use tide::Request;
use super::handler::{handle_error, handle_response};

pub async  fn endpoint_example(req: Request<AppState>) -> tide::Result{
    let pool = &req.state().pool;
    let param = req.param("id").unwrap_or("1");

    match fetches::fetch_Test(pool, param, "").await{
        Ok(employee) => handle_response(employee),
        Err(e) => handle_error(&req, e)
    }
}