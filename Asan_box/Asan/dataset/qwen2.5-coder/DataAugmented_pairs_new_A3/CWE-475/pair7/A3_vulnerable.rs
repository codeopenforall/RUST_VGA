struct MyStruct {
    elems: Vec<i32>,
}

impl MyStruct {
    pub fn new(elems: Vec<i32>) -> Self {
        MyStruct { elems }
    }

    pub fn get(&self, pos: usize) -> i32 {
        unsafe {
            let raw = self.elems.as_ptr();
            *raw.add(pos)
        }
    }
}

pub fn execute(index: usize) {
    let my_struct = MyStruct::new(vec![1, 2, 3, 4, 5]);
    let _ = my_struct.get(index);
}