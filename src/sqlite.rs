use rusqlite::{Connection, Result};

static DB_NAME: &str = "todo.db";

pub fn establish_connection() -> Result<()> {
    let conn = Connection::open(DB_NAME)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS todo (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            description TEXT NOT NULL,
            status INTEGER NOT NULL
        )
        ",
        (),
    )?;

    Ok(())
}
