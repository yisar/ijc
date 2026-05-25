use yjc::parser::{block, statement, expression};

fn main() {
    let input = "for (let x in y) return ; ";
    
    println!("Testing block parse:");
    match block(input) {
        Ok((remaining, node)) => {
            println!("Success! Remaining: '{}'", remaining);
            println!("Node: {:?}", node);
        }
        Err((remaining, err)) => {
            println!("Failed at: '{}'", remaining);
            println!("Error: {:?}", err);
        }
    }
    
    println!("\nTesting statement parse:");
    match statement(input) {
        Ok((remaining, node)) => {
            println!("Success! Remaining: '{}'", remaining);
            println!("Node: {:?}", node);
        }
        Err((remaining, err)) => {
            println!("Failed at: '{}'", remaining);
            println!("Error: {:?}", err);
        }
    }
}
