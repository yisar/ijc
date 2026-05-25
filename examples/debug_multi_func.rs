use yjc::parser::{block, statement};

fn main() {
    let input = "function a(){} function b(){}";
    
    match block(input) {
        Ok((remaining, _node)) => {
            println!("Success! Remaining: '{}'", remaining);
        }
        Err((remaining, err)) => {
            println!("Failed at: '{}'", remaining);
            println!("Error: {:?}", err);
        }
    }
}
