use std::sync::Arc;

struct Object {
    data: i32,
}

impl Object {
    fn new(data: i32) -> Self {
        Object { data }
    }
}

struct Handler {
    handle: Arc<Object>,
}

impl Handler {
    fn new(handle: Arc<Object>) -> Self {
        Handler { handle }
    }

    fn get_data(&self) -> i32 {
        self.handle.data
    }
}

fn run() -> i32 {
    let obj = Arc::new(Object::new(123));
    let handler = Handler::new(Arc::clone(&obj));
    drop(obj);
    handler.get_data()
}