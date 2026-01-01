#![allow(dead_code)]
use std::env;
const MAX_DEPTH: usize = 1000;
#[derive(Debug)]
struct Node {
    children: Vec<Node>,
}
fn build_ast_safe(chars: &mut std::str::Chars, depth: usize) -> Result<Node, String> {
    if depth > MAX_DEPTH {
        return Err("Recursion limit exceeded".to_string());
    }
    let mut node = Node { children: Vec::new() };
    while let Some(ch) = chars.next() {
        if ch == '(' {
            let child = build_ast_safe(chars, depth + 1)?;
            node.children.push(child);
        } else if ch == ')' {
            return Ok(node);
        } else {
        }
    }
    Ok(node)
}
pub fn parse_nested(input: &str) -> Result<Node, String> {
    let mut chars = input.chars();
    build_ast_safe(&mut chars, 0)
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
        Err(e) => println!("Parsing failed with error: {}", e),
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
