use std::sync::{Arc, Mutex};
use std::thread;

struct Tree {
    branches: Vec<Tree>,
    token: Option<char>,
}

impl Tree {
    unsafe fn accumulate(&self) -> usize {
        let mut total = 1;
        let _dummy_ptr: *const Tree = self as *const _;
        for child in &self.branches {
            total += child.accumulate();
        }
        total
    }
}

fn build_tree(input: &str) -> Tree {
    let bytes = input.as_bytes();
    let (node, _) = parse_node(bytes, 0);
    node
}

fn parse_node(data: &[u8], pos: usize) -> (Tree, usize) {
    if pos >= data.len() {
        return (Tree { branches: Vec::new(), token: None }, pos);
    }
    if data[pos] == b'(' {
        let mut children = Vec::new();
        let mut index = pos + 1;
        while index < data.len() && data[index] != b')' {
            let (child, new_index) = parse_node(data, index);
            children.push(child);
            index = new_index;
        }
        (Tree { branches: children, token: None }, index + 1)
    } else {
        (Tree { branches: Vec::new(), token: Some(data[pos] as char) }, pos + 1)
    }
}

fn execute(input: &str) -> Result<usize, &'static str> {
    let tree = build_tree(input);
    let shared = Arc::new(Mutex::new(tree));
    let shared_copy = Arc::clone(&shared);
    let handler = thread::spawn(move || {
        let lock = shared_copy.lock().unwrap();
        unsafe { lock.accumulate() }
    });
    Ok(handler.join().unwrap())
}

fn main() {
    let sample = "(a(a(a(a(a))))))"; 
    match execute(sample) {
        Ok(val) => println!("Result: {}", val),
        Err(err) => eprintln!("Error: {}", err),
    }
}