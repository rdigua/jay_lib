use jay_lib::fns::fn_db;
#[test]
fn test_db() {
    let mut open = false;
    if let Ok(db) = fn_db::c_db("./.temporary/test.db") {
        open = db.is_autocommit();
    }
    assert_eq!(true, open);
}
