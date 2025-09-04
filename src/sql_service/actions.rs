use super::sql_error::SqlServErr;
use sqlx::PgPool;


pub async fn run_query(query: &str, pool: &PgPool) -> Option<SqlServErr> {
    match sqlx::query(query)
        .execute(pool)
        .await{
            Ok(_) => None,
            Err(e) => Some(SqlServErr::SQLError(e.to_string())),
        }
        
}

pub async fn create_table(name: &str, pool: &PgPool, query: &str) -> Option<SqlServErr> {
    match sqlx::query(query)
        .execute(pool)
        .await{
            Ok(_) => None,
            Err(e) => Some(SqlServErr::CreatingTable(name.to_string(),e.to_string())),
        }
} 