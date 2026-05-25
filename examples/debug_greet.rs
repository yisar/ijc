use yjc::parser::{block, statement};

fn main() {
    let input = "function greet(name: string, greeting?: string): string {
  return greeting ? greeting + \" \" + name : \"Hello \" + name;
}";
    
    match block(input) {
        Ok((remaining, _node)) => {
            println!("Success! Remaining length: {}", remaining.len());
            println!("Remaining: '{}'", remaining);
        }
        Err((remaining, err)) => {
            println!("Failed at: '{}'", &remaining[..remaining.len().min(100)]);
            println!("Error: {:?}", err);
        }
    }
}
