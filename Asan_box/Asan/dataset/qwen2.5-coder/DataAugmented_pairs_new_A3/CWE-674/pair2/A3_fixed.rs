struct Node {
    children: Vec<Node>,
}

const MAX_DEPTH: usize = 1000;

fn build_ast_safe(chars: &mut std::str::Chars, depth: usize) -> Result<Node, String> {
    if depth > MAX_DEPTH {
        return Err("Recursion limit exceeded".to_string());
    }

    let mut node = Node { children: Vec::new() };

    while let Some(c) = chars.next() {
        if c == '(' {
            let child = build_ast_safe(chars, depth + 1)?;
            node.children.push(child);
        } else if c == ')' {
            return Ok(node);
        }
    }

    Ok(node)
}

pub fn parse_nested(input: &str) -> Result<Node, String> {
    let mut chars = input.chars();
    build_ast_safe(&mut chars, 0)
}