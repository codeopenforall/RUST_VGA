struct Node {
    children: Vec<Node>,
}

impl Node {
    fn new() -> Self {
        Node { children: Vec::new() }
    }
}

struct Parser<'a> {
    input: &'a str,
    position: usize,
}

impl<'a> Parser<'a> {
    const MAX_DEPTH: usize = 1000;

    fn new(input: &'a str) -> Self {
        Parser { input, position: 0 }
    }

    fn parse_nested(&mut self, depth: usize) -> Result<Node, &'static str> {
        if depth > Self::MAX_DEPTH {
            return Err("Maximum recursion depth exceeded");
        }
        let mut node = Node::new();
        node.children.push(self.parse_nested(depth + 1)?);
        Ok(node)
    }

    fn parse_expr(&mut self) -> Result<Node, &'static str> {
        self.parse_nested(0)
    }
}