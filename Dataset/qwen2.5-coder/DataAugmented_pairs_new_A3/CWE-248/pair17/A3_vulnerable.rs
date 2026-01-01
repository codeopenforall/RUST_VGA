struct Processor;

impl Processor {
    pub fn run(&self) {
        unsafe {
            panic!("Error: Unexpected panic in unsafe operation");
        }
        println!("Processing completed (vulnerable version).");
    }
}