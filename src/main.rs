use errors::ServerError;
use sqlx::PgPool;
use tide::{
    StatusCode,
    http::headers::HeaderValue,
    security::{CorsMiddleware, Origin},
};
use std::sync::{Arc, Mutex};


mod log;
mod sql_service;
mod entities;
mod errors;
mod endpoints;

use log::{LogLevel, Logger, LoggerMiddleware};


#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub logger: Arc<Mutex<Logger>>,
}

#[tokio::main]
async fn main() {
    print!("\x1B[2J\x1B[1;1H"); //clear terminal
    let logger = Arc::new(Mutex::new(Logger::new()));
    let logger_ctrlc = logger.clone();

    let ctrl_c_task = tokio::spawn(async move {
        tokio::signal::ctrl_c().await.expect("failed to listen for ctrl_c");
        println!("Ctrl+C recebido! Salvando logs...");

        if let Ok(mut logger) = logger_ctrlc.lock() {
            if let Err(e) = logger.write_log() {
                logger.add_log_error(&e);
                logger.print_logs();
            }
        }
        std::process::exit(0);
    });


    if let Err(e) = run(logger.clone()).await {
        logger.lock().unwrap().add_log(&e.to_string(), LogLevel::Error);
        if let Err(e) = logger.lock().unwrap().write_log() {
            logger.lock().unwrap().add_log_error(&e);
            logger.lock().unwrap().print_logs();
        }
    }

    let _ = ctrl_c_task.await;
}



async fn run(logger: Arc<Mutex<Logger>>) -> Result<(), ServerError> {
    let database_url = "postgresql://admin:123456@localhost:5433/database?sslmode=disable";
    let ip = "127.0.0.1:5010";
    
    logger.lock()?.add_log(&("Server started at ".to_string() + ip), LogLevel::Info);

    let pool = PgPool::connect(&database_url).await.map_err(|e| {
        logger.lock().unwrap().add_log(&format!("Failed DB connection: {}", e),LogLevel::Error);
        tide::Error::from_str(StatusCode::InternalServerError, "Database failed")
    })?;

    sql_service::tables::setupd_db(&pool).await?;

    let state = AppState {
        pool,
        logger: logger.clone(),
    };

    let mut app = tide::with_state(state);
    app.with(LoggerMiddleware);
    app.with(CorsMiddleware::new()
        .allow_methods("GET, POST, OPTIONS".parse::<HeaderValue>().unwrap())
        .allow_origin(Origin::from("*"))
        .allow_credentials(false)
    );

    app.at("/api/teste").get(endpoints::get::endpoint_example);

    println!("Server running on {}", ip);
    app.listen(ip).await?;
    Ok(())
}