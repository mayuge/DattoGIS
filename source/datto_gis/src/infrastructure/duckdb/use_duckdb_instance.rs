use duckdb::{Connection, Result};

pub struct DuckDbInstance {
    connection: Connection,
}

impl DuckDbInstance {
    /// DuckDBを開く
    pub fn new(path: &str) -> Result<Self> {
        let connection = Connection::open(path)?;

        connection.execute_batch(
            r#"
            INSTALL spatial;
            LOAD spatial;
            "#,
        )?;

        Ok(Self { connection })
    }

    /// インメモリDB
    pub fn in_memory() -> Result<Self> {
        let connection = Connection::open_in_memory()?;

        connection.execute_batch(
            r#"
            INSTALL spatial;
            LOAD spatial;
            "#,
        )?;

        Ok(Self { connection })
    }

    /// Connectionを取得
    pub fn connection(&self) -> &Connection {
        &self.connection
    }
}
