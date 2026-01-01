use std::sync::Arc;

struct Resource {
    data: Arc<i32>,
}

impl Resource {
    fn new(val: i32) -> Self {
        Resource { data: Arc::new(val) }
    }

    fn clone(&self) -> Self {
        Resource { data: Arc::clone(&self.data) }
    }
}

fn main() {
    let resource1 = Resource::new(42);
    let resource2 = resource1.clone();

    // Both resource1 and resource2 share the same data, no need to free manually.
}