use std::io::Write;
use tide::{Request, Next};
use tide::Middleware;
use crate::errors::ServerError;
use crate::AppState;

const LOG_FOLDER : &str = "./logs";

const GREEN : &str = "\x1b[32m";
const RED : &str = "\x1b[31m";
const YELLOW : &str = "\x1b[33m";
const NORMAL : &str = "\x1b[0m";

pub struct Logger{
    pub log_buffer: Vec<String>,
    pub log_file: String,
}

impl Logger{
    
    pub fn new() -> Logger{
        let log_file = match create_log_file(){
            Ok(file) => file,
            Err(e) => panic!("Error creating log file: {}", e),
        };
        Logger{
            log_buffer: Vec::new(),
            log_file: log_file,
        }
    }

    pub fn add_log(&mut self, log: &str, level: LogLevel) {
        let hour = chrono::Local::now().format("%H:%M:%S").to_string();
        
        print_with_level(&hour, &log, &level);
        self.log_buffer.push(hour + "|" + &level.to_string() + " | " + &log);
    }

    pub fn add_log_error(&mut self, server_err : &ServerError){
        self.add_log(&server_err.to_string(), LogLevel::Error);
    }

    pub fn add_request_log(&mut self, path : &str, method : &str){
        let txt = format!("[REQUEST] {} {}", method, path);
        self.add_log(&txt, LogLevel::Info);
    }

    pub fn write_log(&mut self) -> Result<(),crate::ServerError>{
        if !std::path::Path::new(&self.log_file).exists(){
            self.log_file = create_log_file()?;
        }
        self.add_log("writing log into file", LogLevel::Warning);

        let mut file = std::fs::OpenOptions::new()
                                        .write(true)
                                        .append(true)
                                        .open(&self.log_file)?;

        let logs : String = self.log_buffer.iter().map(|log| format!("\n{}", log)).collect();

        file.write_all(logs.as_bytes())?;

        self.log_buffer.clear();

        Ok(())
    }

    pub fn print_logs(&self){
        println!("Error writing log, printing instead");
        let _ = self.log_buffer.iter().map(|log| println!("{}", log));
    }

}

fn print_with_level(hour: &str, log: &str, level: &LogLevel){
    let color = match level {
        LogLevel::Info => GREEN,
        LogLevel::Warning => YELLOW,
        LogLevel::Error => RED
    };
    println!("{}|{}{}{}|{}", hour,color,&level.to_string(),NORMAL,&log);
}

fn create_log_file() -> Result<String, crate::ServerError>{
    let date = chrono::Local::now().format("%Y-%m-%d").to_string();
    
    if !std::path::Path::new(LOG_FOLDER).exists() {
        std::fs::create_dir(LOG_FOLDER)?;
    }

    let log_file_path = format!("{}/{}.log", LOG_FOLDER, date);
    
    std::fs::File::create(log_file_path.clone())?;

    Ok(log_file_path)
}

pub enum LogLevel{
    Info,
    Warning,
    Error,
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warning => write!(f, "WARNING"),
            LogLevel::Error => write!(f, "ERROR"),
        }
    }
} 

#[derive(Debug)]
pub struct LoggerMiddleware;
#[tide::utils::async_trait]
impl Middleware<AppState> for LoggerMiddleware {
    async fn handle(&self, req: Request<AppState>, next: Next<'_, AppState>) -> tide::Result {
        let method = req.method().to_string();
        let path = req.url().path().to_string();


        if let Some(mut logger) = req.state().logger.lock().ok() {
            logger.add_request_log(&path, &method);
        }

        Ok(next.run(req).await)
    }
}