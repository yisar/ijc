use yjc::parser::block;

fn main() {
    let code = r#"function add(a: number, b: number): number {
  return a + b;
}"#;
    
    println!("Parsing: {}", code);
    match block(code) {
        Ok((remaining, _ast)) => {
            println!("Success! Remaining: '{}'", remaining);
        }
        Err((remaining, error)) => {
            println!("Error: {:?} at: '{}'", error, &remaining[..remaining.len().min(50)]);
        }
    }
}
