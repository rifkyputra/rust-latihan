use rusqlite::Connection;

pub fn openConnectionInMemory() -> Connection {
    let conn = Connection::open_in_memory().unwrap();
    conn
}
