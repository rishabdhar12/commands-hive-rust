use rusqlite::{Connection, Result};

pub static DB_NAME: &str = "command.db";
pub static COMMAND_TABLE: &str = "command";

pub fn establish_connection() -> Result<Connection> {
    let conn = Connection::open(DB_NAME)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS command (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            description TEXT NOT NULL
        )
        ",
        (),
    )?;

    Ok(conn)
}
