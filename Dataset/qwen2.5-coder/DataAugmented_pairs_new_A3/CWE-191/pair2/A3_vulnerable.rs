struct Guard;

impl Guard {
    unsafe fn decrease(&self, _deduction: u32) {}
}

struct ValueHolder {
    value: u32,
    guard: Guard,
}

impl ValueHolder {
    unsafe fn decrease(&mut self, deduction: u32) {
        self.value = self.value.wrapping_sub(deduction);
        unsafe {
            self.guard.decrease(deduction);
        }
    }
}

fn run_process() -> u32 {
    let mut holder = ValueHolder {
        value: 50,
        guard: Guard,
    };

    for _ in 0..5 {
        unsafe {
            holder.decrease(20);
        }
    }

    holder.value
}