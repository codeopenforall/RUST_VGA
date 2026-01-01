use std::cell::Cell;
use std::marker::PhantomData;
use std::thread;

/// A generic wrapper that *claims* to be Send for any T,
/// even when T is not Send (like `Cell<T>`).
struct MyWrapper<T> {
    data: *const T,
    _marker: PhantomData<T>,
}

// This is intentionally unsound: T is unconstrained.
unsafe impl<T> Send for MyWrapper<T> {}

fn main() {
    let non_send = Cell::new(0);

    // Store a pointer to a non-Send type inside MyWrapper<T>.
    let wrapper = MyWrapper {
        data: &non_send as *const Cell<i32> as *const _,
        _marker: PhantomData,
    };

    // Move it to another thread. This relies on the unsound `Send` impl.
    let handle = thread::spawn(move || unsafe {
        let cell_ref: &Cell<i32> = &*wrapper.data;
        cell_ref.set(42);
    });

    handle.join().unwrap();
}

