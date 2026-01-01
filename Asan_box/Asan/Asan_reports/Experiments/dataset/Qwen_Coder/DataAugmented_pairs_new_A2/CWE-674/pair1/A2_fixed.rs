use std::sync::{Arc, Mutex};
use std::thread;
const MAX_DEPTH: u32 = 1000;
#[derive(Debug)]
pub struct Node {
    pub value: i32,
    pub children: Vec<Box<Node>>,
}
impl Node {
    pub fn new(val: i32) -> Self {
        Node { value: val, children: Vec::new() }
    }
    pub fn add_child(&mut self, child: Node) {
        self.children.push(Box::new(child));
    }
}
pub unsafe fn traverse_limited(node: &Node, depth: u32, max_depth: u32) -> Option<i32> {
    if depth > max_depth {
        return None;
    }
    let mut total = node.value;
    for child in &node.children {
        match traverse_limited(child, depth + 1, max_depth) {
            Some(child_sum) => total += child_sum,
            None => return None, 
        }
    }
    Some(total)
}
pub fn process_tree(root: &Node) -> Option<i32> {
    unsafe { traverse_limited(root, 0, MAX_DEPTH) }
}
fn main() {
    let mut root = Node::new(1);
    let mut current = &mut root;
    for i in 2..=1500 {
        current.add_child(Node::new(i));
        let last_index = current.children.len() - 1;
        current = current.children[last_index].as_mut();
    }
    let handle = thread::spawn(move || {
        let result = process_tree(&root);
        match result {
            Some(sum) => println!("Sum: {}", sum),
            None => println!("Recursion depth limit reached. Operation aborted."),
        }
        result
    });
    match handle.join() {
        Ok(Some(sum)) => println!("Final Sum: {}", sum),
        Ok(None) => println!("Recursion depth limit reached, operation safely aborted."),
        Err(_) => println!("Thread panicked unexpectedly."),
    }
}

//////////////////////////////////////////////
// Test Oracle for Recursion Vulnerability Contract
//////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    #[test]
    fn recursion_safety_test() {
        // Build a deeply nested tree (chain) to trigger potential uncontrolled recursion.
        let mut root = Node::new(1);
        let mut current = &mut root;
        for i in 2..=1500 {
            current.add_child(Node::new(i));
            let last_index = current.children.len() - 1;
            current = current.children[last_index].as_mut();
        }

        // Use catch_unwind to capture any panic (e.g. stack overflow) during processing.
        let result = panic::catch_unwind(|| process_tree(&root));

        match result {
            Ok(opt) => {
                // In the fixed version, the recursion depth limit should abort the processing, resulting in None.
                // In the vulnerable version, a successful result is unexpected (or may lead to undefined behavior)
                assert!(opt.is_none(), "Expected operation to safely abort (return None) due to recursion depth limit, but got: {:?}", opt);
            },
            Err(_) => {
                panic!("Execution panicked due to uncontrolled recursion vulnerability.");
            }
        }
    }
}
