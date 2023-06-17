use wit_parser::*;
use schemars::schema_for;

#[test]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let schema = schema_for!(Interface);
    let str = serde_json::to_string_pretty(&schema)?;

    std::fs::write("schema.json", str)?;

    Ok(())
}
