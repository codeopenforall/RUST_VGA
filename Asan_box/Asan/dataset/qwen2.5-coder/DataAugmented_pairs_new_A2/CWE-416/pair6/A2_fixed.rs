struct Data {
    value: i32,
}

fn compute() -> i32 {
    let b = Box::new(Data { value: 42 });
    let result = b.value;
    drop(b); // Explicitly drop the Box to free the memory
    result
}

fn main() {
    let res = compute();
    println!("Result: {}", res);
}