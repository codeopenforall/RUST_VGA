struct Info {
    value: i32,
}

impl Info {
    pub fn crunch() -> i32 {
        let original = Box::new(Info { value: 42 });
        let ret = original.value;
        ret
    }
}