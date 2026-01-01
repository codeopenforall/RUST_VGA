use std::sync::{Arc, Mutex};
use std::thread;

struct Tree {
    branches: Vec<Tree>,
    token: Option<char>,
}

impl Tree {
    fn accumulate(&self, depth: usize) -> Result<usize, &'static str> {
        if depth > 1000 {
            return Err("Recursion depth limit exceeded");
        }
        let mut total = 1;
        for child in &self.branches {
            total += child.accumulate(depth + 1)?;
        }
        Ok(total)
    }
}

fn build_tree(input: &str) -> Tree {
    let bytes = input.as_bytes();
    let (node, _) = read_node(bytes, 0);
    node
}

fn read_node(data: &[u8], pos: usize) -> (Tree, usize) {
    if pos >= data.len() {
        return (Tree { branches: Vec::new(), token: None }, pos);
    }
    if data[pos] == b'(' {
        let mut kids = Vec::new();
        let mut idx = pos + 1;
        while idx < data.len() && data[idx] != b')' {
            let (child, new_idx) = read_node(data, idx);
            kids.push(child);
            idx = new_idx;
        }
        (Tree { branches: kids, token: None }, idx + 1)
    } else {
        (Tree { branches: Vec::new(), token: Some(data[pos] as char) }, pos + 1)
    }
}

fn execute(input: &str) -> Result<usize, &'static str> {
    let tree = build_tree(input);
    let shared = Arc::new(Mutex::new(tree));
    let shared_copy = Arc::clone(&shared);
    let worker = thread::spawn(move || {
        let lock = shared_copy.lock().unwrap();
        lock.accumulate(0)
    });
    worker.join().unwrap()
}

fn main() {
    let sample = "(a(a(a(a(a))))))"; 
    match execute(sample) {
        Ok(val) => println!("Result: {}", val),
        Err(err) => eprintln!("Error: {}", err),
    }
}