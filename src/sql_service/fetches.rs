use std::error::Error;

use sqlx::Row;
use sqlx::{PgPool, QueryBuilder};

use crate::entities::jsons::Test;

pub async fn fetch_Test(
    pool: &PgPool, 
    limit : &str,
    filter : &str
) -> Result<Vec<Test>, sqlx::Error> {
    QueryBuilder::new(
        // sql::Select::new()
        // .select("id,name,author,price,genres,in_stock,publisher,storage_id,status")
        // .from("books")
        // .where_clause(if filter != "" {filter} else {"1=1"})
        // .offset(format!("(({}-1) * {})", page, limit).as_str())
        // .limit(&limit)
        ""
    )
    .build_query_as::<Test>()
    .fetch_all(pool)
    .await
}