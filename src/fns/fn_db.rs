//!
//! Now, the functions is to remember.
//! Can you be delect use model of rusqlite.
//!
use rusqlite::Connection;

///Create or open sqlite3 database, open if it exist, create if it not exist.
/// Open a new connection to a SQLite database. If a database does not exist at the path, one is created.
pub fn c_db<S: AsRef<str>>(s: S) -> rusqlite::Result<Connection> {
    let s = s.as_ref();
    let c = Connection::open(s)?;
    Ok(c)
}

///It execute a sql sentence
pub fn sql_execute<S: AsRef<str>>(db: Connection, sql: S) -> rusqlite::Result<()> {
    let s = sql.as_ref();
    db.execute(s, ())?;
    Ok(())
}

///It batch execute a sql.
///
///
pub fn sql_execute_batch<S: AsRef<str>>(db: Connection, sql: S) -> rusqlite::Result<()> {
    let s = sql.as_ref();
    db.execute_batch(s)?;
    Ok(())
}
