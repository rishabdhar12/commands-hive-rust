use rusqlite::{Connection, Result};

use crate::sqlite;

use cli_table::{Cell, Style, Table, format::Justify, print_stdout};

#[derive(Debug)]
pub struct Todo {
    pub id: Option<i32>,
    pub title: String,
    pub description: String,
}

impl Todo {
    pub fn add_todo(todo: &Todo, conn: &Connection) -> Result<()> {
        let _ = conn.execute(
            "INSERT INTO todo (title, description) VALUES (?1, ?2)",
            (&todo.title, &todo.description),
        );

        Ok(())
    }

    pub fn list_todo(conn: &Connection) -> Result<()> {
        let query = format!("SELECT * FROM {}", sqlite::TODO_TABLE);

        let mut stmt = conn.prepare(&query)?;

        let rows = stmt.query_map([], |row| {
            let id: i32 = row.get(0)?;
            let title: String = row.get(1)?;
            let desc: String = row.get(2)?;

            Ok((id, title, desc))
        })?;

        let mut table = vec![];

        for item in rows {
            let (id, title, desc) = item?;

            table.push(vec![
                id.cell(),
                title.cell().justify(Justify::Center),
                desc.cell().justify(Justify::Center),
            ]);
        }

        let table = table
            .table()
            .title(vec![
                "ID".cell().bold(true).justify(Justify::Center),
                "TITLE".cell().bold(true).justify(Justify::Center),
                "DESC".cell().bold(true).justify(Justify::Center),
            ])
            .bold(true);

        assert!(print_stdout(table).is_ok());
        Ok(())
    }


    pub fn update_todo(id: i32, todo: Todo) -> Result<(), String> {
        todo!()
    }

    pub fn delete_todo(id: i32) -> Result<(), String> {
        todo!()
    }
}
