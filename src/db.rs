use rusqlite::{params, Connection, Result};

pub struct Task{
    pub id: i32,
    pub title: String,
    pub completed: bool,
}
pub struct Database{
    conn: Connection
}

impl Database{
    pub fn new(path: &str) -> rusqlite::Result<Self> {
        let conn = Connection::open(path)?;
        //Change autoincrement to uuid?
        conn.execute(
            "CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            completed BOOLEAN NOT NULL DEFAULT 0
        )", 
        (),
        )?;
        Ok(Database {conn})
    }

    pub fn add_task(&self, title: &str) -> rusqlite::Result<i32> {
        if title.trim().is_empty() {
            return Err(rusqlite::Error::InvalidParameterName(
                "Title cannot be empty".to_string()
            ));
        }

        self.conn.execute(
            "INSERT INTO tasks (title, completed) VALUES (?1, 0)",
            params![title],
        )?;

        Ok(self.conn.last_insert_rowid() as i32)    
    }

    pub fn get_task(&self) -> rusqlite::Result<Vec<Task>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, completed FROM tasks ORDER by id"
        )?;

        let tasks = stmt.query_map([], |row| {
            Ok(Task {
                id: row.get(0)?,
                title: row.get(1)?,
                completed: row.get(2)?,
            })
        })?
        .collect::Result<Vec<_>, _>>()?;
        Ok(tasks)
    }
}
