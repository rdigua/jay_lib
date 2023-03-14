use std::fs;
use std::error::Error;
use chrono::{Datelike, Timelike, Utc};

///It is a method to aid memory for me.
type Result<String> = ::std::result::Result<String, Box<dyn Error>>;

///Read string from file.
pub fn f_string<P: AsRef<str>>(s: P) -> Result<String> {
    let r = fs::read_to_string(s.as_ref())?;
    Ok(r)
}

///get a file name from today.
/// Often use for temporary file.
pub fn get_file_date_name() -> String {
    let mut r: String =String::new();
    let now = Utc::now();
    r.push_str(&now.year().to_string());
    r.push_str(&now.month().to_string());
    r.push_str(&now.day().to_string());
    r
}
///get a file name from now.
/// it is year month day hour and minute.
/// Often use for temporary file.
pub fn get_file_time_name() -> String {
    let mut r= String::new();
    let now = Utc::now();
    let add_str = now.year().to_string()
        + &now.month().to_string()
        + &now.day().to_string()
        + &now.hour().to_string()
        + &now.minute().to_string();
    r.push_str(&add_str);
    r
}

///writen `Vec<String>` to file.
pub fn v_f(v:Vec<String>) -> std::io::Result<()> {
    let mut s=String::from("");
    for i in v{
        s.push_str(&i);
        s.push('\n');
    }
    fs::write("./data/dup_rows", s)?;
    Ok(())
}