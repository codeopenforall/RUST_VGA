struct Dispatcher {
    safe_mode: bool,
    table: [fn(i32) -> i32; 2],
}

impl Dispatcher {
    fn new(safe_mode: bool) -> Self {
        Dispatcher {
            safe_mode,
            table: [nonsensitive, sensitive],
        }
    }

    fn process(&self, input: i32) -> i32 {
        if self.safe_mode {
            self.table[0](input)
        } else {
            self.table[input as usize % 2](input)
        }
    }
}

fn nonsensitive(x: i32) -> i32 {
    x + 5
}

fn sensitive(x: i32) -> i32 {
    x * 10
}