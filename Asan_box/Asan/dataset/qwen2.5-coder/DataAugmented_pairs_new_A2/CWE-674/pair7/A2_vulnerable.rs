struct Node {
    children: Vec<Node>,
}

impl Node {
    unsafe fn new() -> Self {
        Node { children: Vec::new() }
    }
}

struct Parser<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Parser { input, pos: 0 }
    }

    fn parse_nested(&mut self) -> Result<Node, &'static str> {
        let mut node;
        unsafe {
            node = Node::new();
        }
        while self.pos < self.input.len() && self.input.chars().nth(self.pos).unwrap() == '(' {
            self.pos += 1;
            node.children.push(self.parse_nested()?);
        }
        if self.pos < self.input.len() && self.input.chars().nth(self.pos).unwrap() == ')' {
            self.pos += 1;
        } else {
            return Err("Mismatched parentheses");
        }
        Ok(node)
    }

    fn parse_expr(&mut self) -> Result<Node, &'static str> {
        self.parse_nested()
    }
}