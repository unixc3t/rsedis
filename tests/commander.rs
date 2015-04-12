extern crate rsedis;

use std::str::from_utf8;

use rsedis::database::Database;
use rsedis::database::Value;
use rsedis::parser::Parser;
use rsedis::parser::Argument;
use rsedis::commander::run;
use rsedis::commander::Response;

fn getstr(database: &Database, key: &[u8]) -> String {
    match database.get(&key.to_vec()) {
        Some(val) => {
            match val {
                &Value::Data(ref bytes) => return from_utf8(bytes).unwrap().to_string(),
            }
        },
        _ => assert!(false),
    }
    return String::new();
}

#[test]
fn nocommand() {
    let mut db = Database::new();
    let parser = Parser::new(b"", 0, Vec::new());
    let response = run(&parser, &mut db);
    match response {
        Response::Err(err) => {},
        _ => assert!(false),
    };
}

#[test]
fn set_command() {
    let mut db = Database::new();
    let parser = Parser::new(b"setkeyvalue", 3, vec!(
                Argument {pos: 0, len: 3},
                Argument {pos: 3, len: 3},
                Argument {pos: 6, len: 5},
                ));
    let response = run(&parser, &mut db);
    match response {
        Response::Status(msg) => assert_eq!(msg, "OK"),
        _ => assert!(false),
    };
    assert_eq!("value", getstr(&db, b"key"));
}
