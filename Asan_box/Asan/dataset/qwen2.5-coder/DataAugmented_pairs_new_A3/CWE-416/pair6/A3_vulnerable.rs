struct Data {
    value: i32,
}
fn compute() -> i32 {
    let b = Box::new(Data { value: 42 });
    let ptr = Box::into_raw(b);
    unsafe {
        drop(Box::from_raw(ptr));
    }
    let result = unsafe { (*ptr).value };
    result
}
fn main() {
    let res = compute();
    println!("Result: {}", res);
}