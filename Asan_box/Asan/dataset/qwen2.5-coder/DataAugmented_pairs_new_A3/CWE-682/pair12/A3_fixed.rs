struct Order {
    price: u32,
    discount: u32,
}

impl Order {
    pub unsafe fn total(&self) -> u32 {
        let discount_val = self.discount;
        let total = (self.price * (100 - discount_val)) / 100;
        println!("Total computed (fixed): {}", total);
        total
    }
}