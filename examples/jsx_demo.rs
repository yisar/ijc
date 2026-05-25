use yjc::parser::block;
use yjc::printer::Printer;

fn main() {
    let examples = vec![
        ("Basic JSX", r#"const el = <div>Hello World</div>;"#),
        ("Self-closing", r#"const img = <img src="test.jpg" />;"#),
        ("Fragment", r#"const frag = <><div>A</div><div>B</div></>;"#),
        ("Member Expression", r#"const el = <React.Fragment>Hello</React.Fragment>;"#),
        ("Namespaced", r#"const svg = <svg:path d="M0 0" />;"#),
        ("Spread Props", r#"const el = <Component {...props} />;"#),
        ("Hyphenated Tag", r#"const el = <my-component>Content</my-component>;"#),
        ("Complex Example", r#"
const App = (
  <Container>
    <Header title="Test" />
    <Content>
      {items.map(item => (
        <Item key={item.id} {...item} />
      ))}
    </Content>
    <Footer />
  </Container>
);
"#),
    ];

    for (name, code) in examples {
        println!("=== {} ===", name);
        println!("Input:\n{}\n", code);
        
        match block(code) {
            Ok((remaining, ast)) => {
                if !remaining.trim().is_empty() {
                    println!("Warning: {} characters remaining", remaining.len());
                }
                
                let mut printer = Printer::new("  ", "\n");
                printer.print(&ast);
                println!("Output:\n{}\n", printer.code());
            }
            Err((remaining, error)) => {
                println!("Error: {:?} at: {}\n", error, &remaining[..remaining.len().min(50)]);
            }
        }
        println!();
    }
}
