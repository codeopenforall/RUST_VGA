struct Processor;

impl Processor {
    pub fn operate(&self, input: &[u32]) -> Option<Vec<u32>> {
        if input.len() < 1 {
            return None;
        }
        let data: Vec<u32> = vec![42];
        Some(data)
    }
}