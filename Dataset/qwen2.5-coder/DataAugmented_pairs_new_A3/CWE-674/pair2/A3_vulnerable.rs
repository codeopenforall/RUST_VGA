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
    while let Some(c) = chars.next() {
        if c == '(' {
            let child = build_ast(chars);
            node.children.push(child);
        } else if c == ')' {
            break;
        }
    }
    node
}

pub fn parse_nested(input: &str) -> Result<Node, ()> {
    let mut chars = input.chars();
    let ast = build_ast(&mut chars);
    Ok(ast)
}