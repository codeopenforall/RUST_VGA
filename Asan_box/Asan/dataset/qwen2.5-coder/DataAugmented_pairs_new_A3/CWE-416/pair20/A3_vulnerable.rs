struct Object {
    data: i32,
}

impl Object {
    fn new(data: i32) -> Self {
        Object { data }
    }
}

struct Handler {
    ptr: *const Object,
}

impl Handler {
    fn new(raw: *const Object) -> Self {
        Handler { ptr: raw }
    }

    fn get_data(&self) -> i32 {
        unsafe { (*self.ptr).data }
    }
}

fn run() -> i32 {
    let obj = Box::new(Object::new(123));
    let raw_ptr = Box::into_raw(obj);
    let handler = Handler::new(raw_ptr);
    unsafe {
        Box::from_raw(raw_ptr);
    }
    handler.get_data()
}