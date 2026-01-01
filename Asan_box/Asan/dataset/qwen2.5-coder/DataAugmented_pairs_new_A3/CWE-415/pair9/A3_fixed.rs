struct Item {
    data: *mut i32,
}

impl Item {
    fn from_value(val: i32) -> Self {
        let boxed = Box::new(val);
        Item { data: Box::into_raw(boxed) }
    }

    fn value(&self) -> i32 {
        unsafe { *self.data }
    }

    fn drop(&mut self) {
        let val = self.value();
        Item::from_value(val);
        drop(unsafe { Box::from_raw(self.data) });
    }
}

fn run_app() {
    let mut item = Item::from_value(42);
    item.drop();
    // Attempting to drop again would normally cause a double free, but the fix prevents this.
    item.drop();
}