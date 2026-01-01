struct Dispatcher {
    table: Vec<fn(i32) -> i32>,
    safe_mode: bool,
}

impl Dispatcher {
    pub fn new(safe_mode: bool) -> Self {
        Dispatcher {
            table: vec![nonsensitive, sensitive],
            safe_mode,
        }
    }

    pub fn process(&self, input: i32) -> i32 {
        unsafe {
            let func_ptr = *self.table.get_unchecked(input as usize % self.table.len());
            func_ptr(input)
        }
    }
}

fn nonsensitive(x: i32) -> i32 {
    x + 5
}

fn sensitive(x: i32) -> i32 {
    x * 10
}