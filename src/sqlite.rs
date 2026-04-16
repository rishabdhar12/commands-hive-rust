use rusqlite::{Connection, Result};

pub static DB_NAME: &str = "todo.db";
pub static TODO_TABLE: &str = "todo";

pub fn establish_connection() -> Result<Connection> {
    let conn = Connection::open(DB_NAME)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS todo (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            description TEXT NOT NULL
        )
        ",
        (),
    )?;

    Ok(conn)
}
