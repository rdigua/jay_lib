use std::fs;
use std::error::Error;
use chrono::{Datelike, Timelike, Utc};

///It is a method to aid memory for me.
type Result<String> = ::std::result::Result<String, Box<dyn Error>>;

///Read string from file.
/// Example:
///
/// ```rust
/// # use std::error::Error;
/// # use once_cell::sync::OnceCell;
/// # use toml::Value;
/// # use jay_lib::fns::fn_io::f_string;
/// # type Result<> = ::std::result::Result<(), Box<dyn Error>>;
/// # pub static DB_LOCATION: OnceCell<String> = OnceCell::new();
/// # pub static DICTIONARY_FROM: OnceCell<String> = OnceCell::new();
/// # pub static RUST_FROM: OnceCell<String> = OnceCell::new();
/// #
/// # pub fn set_conf() -> Result<> {
///     let mut s = String::from("");
///     match f_string("./config.toml") {
///         Ok(st) => s = st,
///         Err(e) => eprintln!("{e}"),
///     }
/// #   let toml_info: Value = toml::from_str(&s)?;
/// #   if let Some(from) = toml_info["data"]["dictionary_from"].as_str() {
/// #        DICTIONARY_FROM.get_or_init(|| from.to_string());
/// #   }
/// #   if let Some(tidy) = toml_info["data"]["rust_from"].as_str() {
/// #       RUST_FROM.get_or_init(|| tidy.to_string());
/// #   }
/// #   if let Some(location) = toml_info["db"]["location"].as_str() {
/// #       DB_LOCATION.get_or_init(|| location.to_string());
/// #   }
/// #   Ok(())
/// }
/// ```
pub fn f_string<P: AsRef<str>>(s: P) -> Result<String> {
    let r = fs::read_to_string(s.as_ref())?;
    Ok(r)
}

///get a file name from today.
/// Often use for temporary file.
///```rust
/// # use std::fs;
/// # use jay_lib::fns::fn_io::get_file_date_name;
/// # fn main()->std::io::Result<()> {
/// let p=get_file_date_name();
/// let s="12345";
/// fs::write(p, s)?;
/// # Ok(())
/// # }
/// ```
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
///```rust
/// # use std::fs;
/// # use jay_lib::fns::fn_io::get_file_time_name;
/// # fn main()->std::io::Result<()> {
/// let p=get_file_time_name();
/// let s="12345";
/// fs::write(p, s)?;
/// # Ok(())
/// # }
///
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
///writen `Vec<String>` to a temporary file.
/// It to check temporary data.
///```no_run
/// use jay_lib::fns::fn_io::v_f;
///    let v=["1".to_string(),"2".to_string(),"3".to_string()];
///   if let Err(_) = v_f(v.to_vec()) {
///         eprintln!("Save v to file has some err");
///   }
/// ```
pub fn v_f(v: Vec<String>) -> std::io::Result<()> {
    let mut s = String::from("");
    for i in v {
        s.push_str(&i);
        s.push('\n');
    }
    let mut p = ::std::string::String::from("./t/");
    p.push_str(&get_file_time_name());
    fs::write(p, s)?;
    Ok(())
}
///Writen a String to a file.
///```no_run
///
///    use jay_lib::fns::fn_io::s_f;
///      //s is a sql string.
///    let s="Inst into xxxx".to_string();
///    if let Err(_) = s_f(s) {
///         eprintln!("Save v to file has some err");
///     }
/// ```
pub fn s_f(s:String) -> std::io::Result<()> {
    let mut p = ::std::string::String::from("./t/");
    p.push_str(&get_file_time_name());
    fs::write(p, s)?;
    Ok(())
}
