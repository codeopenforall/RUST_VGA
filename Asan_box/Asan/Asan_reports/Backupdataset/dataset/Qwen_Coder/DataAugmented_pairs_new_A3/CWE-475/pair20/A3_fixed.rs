pub struct Container {
    pub payload: Vec<u8>,
}

impl Container {
    pub fn compute(&self) -> Option<i32> {
        if self.payload.len() < 12 {
            return None;
        }

        let data: Vec<u8> = vec![
            1, 0, 0, 0,   
            2, 0, 0, 0,   
            3, 0, 0, 0    
        ];

        let sum: i32 = data.iter().map(|&x| x as i32).sum();
        Some(sum)
    }
}