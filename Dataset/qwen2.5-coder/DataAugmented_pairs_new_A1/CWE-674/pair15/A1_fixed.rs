use std::sync::{Arc, Mutex};
use std::thread;
#[derive(Debug)]
enum Ast {
    Node(Box<Ast>),
    Leaf(char),
}
struct Parser {
    input: Vec<char>,
    pos: usize,
    depth: usize,
}
impl Parser {
    fn new(s: &str) -> Self {
        Self {
            input: s.chars().collect(),
            pos: 0,
            depth: 0,
        }
    }
    fn parse_expr(&mut self) -> Result<Ast, String> {
        if self.pos >= self.input.len() {
            return Err("unexpected end".into());
        }
        let c = self.input[self.pos];
        self.pos += 1;
        if c == '(' {
            self.depth += 1;
            if self.depth > 1000 {
                return Err("recursion limit exceeded".into());
            }
            let node = self.parse_expr()?; 
            if self.pos >= self.input.len() {
                return Err("expected ')' but found end".into());
            }
            let c2 = self.input[self.pos];
            if c2 != ')' {
                return Err("expected ')'".into());
            }
            self.pos += 1;
            self.depth -= 1;
            Ok(Ast::Node(Box::new(node)))
        } else {
            Ok(Ast::Leaf(c))
        }
    }
}
fn execute(input: &str) -> Result<Ast, String> {
    let mut parser = Parser::new(input);
    parser.parse_expr()
}
fn main() {
    let input = "((".to_owned() + &"(".repeat(2000) + "x" + &")".repeat(2000) + ")";
    let result_arc = Arc::new(Mutex::new(None));
    let result_clone = Arc::clone(&result_arc);
    let input_clone = input.clone();
    let handle = thread::spawn(move || {
        let res = execute(&input_clone);
        let mut guard = result_clone.lock().unwrap();
        *guard = Some(res);
    });
    handle.join().unwrap();
    let guard = result_arc.lock().unwrap();
    match &*guard {
        Some(Ok(ast)) => println!("Parsed AST: {:?}", ast),
        Some(Err(e)) => println!("Error during parsing: {}", e),
        None => println!("No result produced"),
    }
}