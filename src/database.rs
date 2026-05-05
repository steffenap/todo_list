use rusqlite::{Connection, Result};


#[derive(Debug)]
struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub unique_id: String,
}

#[derive(Debug)]
struct Task {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
    pub user: Option<i32>,
}


fn create_tables(conn: &Connection) -> Result<()> {
    match conn.execute(
        "CREATE TABLE IF NOT EXISTS users(
            id INTEGER PRIMARY KEY SERIAL NOT NULL,
            username VARCHAR(32) NOT NULL,
            email VARCHAR(64) NOT NULL,
            password VARCHAR(256) NOT NULL,
            unique_id VARCHAR(36) NOT NULL
        )",
        (),
    ) {
        Ok(_) => {
            println!("Created table");
        }
        Err(err) => return Err(err.into())
    }

    match conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks(
            id INT AUTOINCREMENT PRIMARY KEY,
            title TEXT NOT NULL,
            description TEXT,
            completed BOOLEAN,
            user ID FOREIGN KEY(users(id)) NOT NULL
        )",
        (),
    ) {
        Ok(_) => {
            println!("Created table");
        }
        Err(err) => return Err(err.into()),
    }
    Ok(())
}

fn insert_task(conn: &Connection, task: &Task) -> Result<(), rusqlite::Error> {
    match conn.execute(
        "",
        (&task.id, &task.title, &task.description, &task.completed, &task.user)
    )
}