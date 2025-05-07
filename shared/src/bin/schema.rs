use shared::User;
use schemars::schema_for;
use std::fs::File;
use std::io::Write;

fn main() {
    let schema = schema_for!(User);
    let json = serde_json::to_string_pretty(&schema).unwrap();
    let mut file = File::create("schema.json").unwrap();
    file.write_all(json.as_bytes()).unwrap();
}
