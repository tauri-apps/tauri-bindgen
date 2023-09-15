use schemars::schema_for;
use wit_parser::*;

#[test]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let schema = schema_for!(Interface);
    let str = serde_json::to_string_pretty(&schema)?;

    std::fs::write("ast.json", str)?;

    Ok(())
}
