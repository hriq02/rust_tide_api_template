use crate::AppState;
use serde::Serialize;
use tide::{Request, Response, StatusCode};

pub fn handle_response<T: Serialize>(t: T) -> tide::Result{
    Ok(Response::builder(StatusCode::Ok)
        .body(serde_json::to_string(&t)?)
        .content_type("application/json")
        .build()
    )
}

pub fn handle_error(req: &Request<AppState>, e: sqlx::Error) -> tide::Result {
    let error = e.to_string();
    match req.state().
    logger.lock(){
        Ok(mut logger) => 
            logger.add_log_error(&e.into()),
        Err(e) => 
            println!("Failed to log error: {}", e),
    }
    Ok(Response::builder(StatusCode::InternalServerError)
        .body(format!("Failed to fetch: {:?}", error))
        .build()
    )
}