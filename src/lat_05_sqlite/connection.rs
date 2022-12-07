use core::time;
use std::path::Path;

use rusqlite::{backup, Connection, Error};

pub fn open_connection_in_memory() -> Connection {
    let conn = Connection::open_in_memory().unwrap();
    conn
}

// pub fn export_sqlite_as_file(conn: &Connection, path: &str) -> Result<(), rusqlite::Error> {
//     conn.conn.backup_file("db.sqlite").unwrap();
// }

pub fn backup_db<P: AsRef<Path>>(
    src: &Connection,
    dst: P,
    progress: fn(backup::Progress),
) -> Result<(), Error> {
    let mut dst = Connection::open(dst)?;
    let backup = backup::Backup::new(src, &mut dst)?;
    backup.run_to_completion(5, time::Duration::from_millis(600), Some(progress))
}

// load database from file
pub fn load_db<P: AsRef<Path>>(src: P) -> Result<Connection, Error> {
    let conn = Connection::open(src)?;
    Ok(conn)
}
