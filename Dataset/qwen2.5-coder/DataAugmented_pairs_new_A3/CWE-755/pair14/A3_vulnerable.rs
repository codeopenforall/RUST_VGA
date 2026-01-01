use std::sync::Arc;

struct Engine {
    base: i32,
}

impl Engine {
    fn new(base: i32) -> Self {
        Engine { base }
    }

    fn process(&self, input: &str) -> i32 {
        let divisor: i32 = input.parse().unwrap_or_default();
        self.base / divisor
    }
}

fn perform(input: &str) -> Result<i32, String> {
    let engine = Arc::new(Engine::new(100));
    let engine = Arc::clone(&engine);
    let res = engine.process(&input);
    Ok(res)
}