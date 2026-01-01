use std::sync::Arc;

struct Data {
    arr: Vec<i32>,
}

impl Data {
    fn new() -> Self {
        Data { arr: vec![10, 20, 30, 40, 50, 60, 70] }
    }
}

struct Engine {
    data: Arc<Data>,
}

impl Engine {
    fn new(data: Arc<Data>) -> Self {
        Engine { data }
    }

    fn execute(&self, pos: usize) -> Result<i32, &'static str> {
        if pos < 5 {
            return Err("Input value too small, causes underflow");
        }
        let index = pos - 5; 
        if index >= self.data.arr.len() {
            return Err("Index out of bounds");
        }
        Ok(self.data.arr[index])
    }
}

fn main() {
    let data = Arc::new(Data::new());
    let engine = Engine::new(data);

    // Example usage
    match engine.execute(7) {
        Ok(val) => println!("Value at position 7: {}", val),
        Err(msg) => println!("Error: {}", msg),
    }
}