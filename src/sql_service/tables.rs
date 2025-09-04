use sqlx::PgPool;
use super::{actions::run_query, actions::create_table, sql_error::SqlServErr};

pub async fn setupd_db(pool: &PgPool) -> Result<(),SqlServErr>{

    Ok(())
}