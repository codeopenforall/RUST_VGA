struct MyStruct {
    elems: Vec<i32>,
}

impl MyStruct {
    pub fn new(elems: Vec<i32>) -> Self {
        MyStruct { elems }
    }

    pub fn get(&self, pos: usize) -> i32 {
        if pos < self.elems.len() {
            self.elems[pos]
        } else {
            panic!("Provided index {} is out-of-bound (max index {}).", pos, self.elems.len() - 1);
        }
    }
}

pub fn execute(index: usize) {
    let my_struct = MyStruct::new(vec![1, 2, 3, 4, 5]);
    my_struct.get(index);
}