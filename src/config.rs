//! To me, OnceCell is a good tool for config.
//! It said:
//! A cell which can be written to only once.
//! Ok!
//! Call config from config add institute.
//!

use crate::fns::fn_io;
use once_cell::sync::OnceCell;
use std::error::Error;
use toml::Value;

type Result = ::std::result::Result<(), Box<dyn Error>>;

/// Often, it can be a database url or file.
pub static DB_LOCATION: OnceCell<String> = OnceCell::new();
/// and it can be a source from what?
pub static DATA_FROM: OnceCell<String> = OnceCell::new();
/// Ordering temporary place.
pub static DATA_TIDY: OnceCell<String> = OnceCell::new();

///Open file
///get fields which is used.
///written to only once
///used it for main or lib.
///
/// It is just a example for me.
///Can you be look source code and changed it for yourself.
///
/// Example:
///
///```rust
///    use jay_lib::config::{set_conf,DB_LOCATION,DATA_FROM,DATA_TIDY};
///    let db_location;
///    let dic_from;
///    let tidy_data;
///    match set_conf() {
///        Ok(_) => {
///            db_location = DB_LOCATION.get();
///            dic_from = DATA_FROM.get();
///            tidy_data = DATA_TIDY.get();
///            println!("{:?},{:?},{:?}", db_location, dic_from, tidy_data);
///        }
///        Err(e) => { eprintln!("{e:?}");return; }
///    };
/// ```
/// ***It will be changed.***
/// If no exist config file ... ***To Do***
///
pub fn set_conf() -> Result {
    let mut s = String::from("");
    if let Ok(st) = fn_io::f_string("./.temporary/config.toml") {
        s = st;
    }
    let toml_info: Value = toml::from_str(&s)?;
    if let Some(from) = toml_info["data"]["from"].as_str() {
        DATA_FROM.get_or_init(|| from.to_string());
    }
    if let Some(tidy) = toml_info["data"]["tidy"].as_str() {
        DATA_TIDY.get_or_init(|| tidy.to_string());
        //data_tidy=tidy;
    }
    if let Some(location) = toml_info["db"]["location"].as_str() {
        DB_LOCATION.get_or_init(|| location.to_string());
    }
    Ok(())
}
