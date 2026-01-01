struct Data {
    pub input: String,
}

trait Executor {
    fn execute(&self) -> String;
}

impl Executor for Data {
    fn execute(&self) -> String {
        format!("{}", self.input)
    }
}

fn main() {
    println!("Usage: program <input>");
}