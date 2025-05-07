use schemars::schema_for;
use shared::models::User; // adjust if the module path is different
use std::fs::File;
use std::io::Write;

fn main() {
    let schema = schema_for!(User);
    let json = serde_json::to_string_pretty(&schema).unwrap();

    let mut file = File::create("schema.json").unwrap();
    file.write_all(json.as_bytes()).unwrap();
}
