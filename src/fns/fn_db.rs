use ruslite::{Connection};
pub fn c_db<S: AsRef<str>>(s: S) -> rusqlite::Result<Connection> {
    let s = s.as_ref();
    let c = Connection::open(path)?;
    Ok(c)
}

pub fn sql_execute<S:AsRef<str>>(db:Connection,sql:S)-> rusqlite::Result<()>{
    let s=sql.as_ref();
    db.execute(
        s,
        (),
    )?;
    Ok(())
}

pub fn sql_execute_batch<S:AsRef<str>>(db:Connection,sql:S)->rusqlite::Result<()>{
    let s=sql.as_ref();
    db.execute_batch(s)?;
    Ok(())
}
/*
pub fn open_my_db<P:AsRef<Path>>(s: P) -> Result<Connection>
{
    let path=s.as_ref();
    //let path = "./my_db.db3";
    let db = Connection::open(path)?;
    // Use the database somehow...
    println!("{}", db.is_autocommit());
    Ok(db)
}

 */