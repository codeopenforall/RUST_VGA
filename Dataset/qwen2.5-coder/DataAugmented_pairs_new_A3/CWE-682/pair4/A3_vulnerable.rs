pub struct Purchase {
    pub price: u32,
    pub discount: u32,
    pub tax: u32,
}

impl Purchase {
    pub fn calculate(&self) -> u32 {
        let discount_val = self.price * self.discount / 100;
        let discounted_price = self.price - discount_val;
        let tax_val = discounted_price / 100 * self.tax; // Vulnerable line
        discounted_price + tax_val
    }
}