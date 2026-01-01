#![allow(dead_code)]
use std::env;
#[derive(Debug)]
struct Node {
    children: Vec<Node>,
}
impl Node {
    unsafe fn alloc() -> *mut Node {
        let node = Box::new(Node { children: Vec::new() });
        Box::into_raw(node)
    }
    unsafe fn dealloc(ptr: *mut Node) {
        if !ptr.is_null() {
            drop(Box::from_raw(ptr));
        }
    }
}
fn build_ast(chars: &mut std::str::Chars) -> Node {
    let mut node = Node { children: Vec::new() };
    unsafe {
        let ptr: *mut Node = &mut node;
        *ptr = Node { children: Vec::new() };
    }
    while let Some(ch) = chars.next() {
        if ch == '(' {
            let child = build_ast(chars);
            node.children.push(child);
        } else if ch == ')' {
            return node;
        } else {
        }
    }
    node
}
pub fn parse_nested(input: &str) -> Result<Node, ()> {
    let mut chars = input.chars();
    let ast = build_ast(&mut chars);
    Ok(ast)
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let input = if args.len() > 1 {
        args[1].clone()
    } else {
        let opens = "(".repeat(1200);
        let closes = ")".repeat(1200);
        opens + &closes
    };
    match parse_nested(&input) {
        Ok(ast) => println!("Parsed AST: {:?}", ast),
        Err(_) => println!("Parsing failed."),
    }
}

/*
This test oracle is designed to verify the behavior of the parsing API.
It supplies an input with a deep nesting level.
For the original (vulnerable) version, the extremely deep input triggers a stack overflow (panic),
causing the test to fail. For the corrected version, the recursion depth check prevents an overflow
and instead returns an error, so the test passes.

Note: This test oracle function is separate from the main code. It uses catch_unwind
to detect panics in the vulnerable code.
*/
#[cfg(test)]
mod tests {
    use super::parse_nested;
    use std::panic;

    #[test]
    fn test_recursion_limit() {
        let deep_input = "(".repeat(1500) + &")".repeat(1500);
        let result = panic::catch_unwind(|| {
            parse_nested(&deep_input)
        });
        match result {
            // For the fixed version, parse_nested returns a Result without panicking.
            Ok(inner) => {
                // Expect an error from excessive recursion.
                // Using a generic match since error type differs between versions.
                if let Ok(_) = inner {
                    panic!("Expected error due to recursion depth, but got a valid AST");
                }
            }
            // For the vulnerable version, a panic occurs (e.g. stack overflow) causing the test to fail.
            Err(_) => {
                panic!("Test failed: uncontrolled recursion caused a panic (stack overflow)");
            }
        }
    }
}
