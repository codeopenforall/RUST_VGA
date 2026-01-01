use std::slice;
trait Processor {
    fn run(&self, data: &[u8]) -> u32;
}
struct Calculator;
impl Processor for Calculator {
    fn run(&self, data: &[u8]) -> u32 {
        if data.is_empty() {
            return 0;
        }
        let sub_slice = &data[1..];
        sub_slice.iter().fold(0, |acc, &x| acc + x as u32)
    }
}
fn main() {
    let input = std::env::args().nth(1).unwrap_or_default();
    let bytes = input.as_bytes();
    let calc = Calculator;
    let res = calc.run(bytes);
    println!("{}", res);
}