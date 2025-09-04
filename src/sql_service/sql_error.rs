

#[derive(Debug)]
pub enum SqlServErr{
    CreatingTable(String,String),
    SQLError(String)
}

impl std::fmt::Display for SqlServErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SqlServErr::CreatingTable(table_name, err) => {
                write!(f, "Error creating table {}\n Error: {}", table_name, err)
            }
            SqlServErr::SQLError(err) => {
                write!(f, "SQL Error: {}", err)
            }
        }
    }
}

impl From<sqlx::Error> for SqlServErr {
    fn from(err: sqlx::Error) -> Self {
        SqlServErr::SQLError(err.to_string())
    }
}
