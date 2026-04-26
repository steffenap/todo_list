pub struct Database{
    conn: Connection
}


pub struct Task{
    pub id: i32,
    pub title: String,
    pub desc: String,
    pub completed: bool,
    pub assignee: User,
}