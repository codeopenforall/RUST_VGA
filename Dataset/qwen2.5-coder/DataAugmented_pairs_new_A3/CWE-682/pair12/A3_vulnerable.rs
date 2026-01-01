struct Order {
    price: u32,
    discount: u32,
}

impl Order {
    pub unsafe fn total(&self) -> u32 {
        let discount_val = self.discount;
        let total = self.price * 100 - discount_val / 100;
        println!("Total computed (vulnerable): {}", total);
        total
    }
}

fn main() {
    let order = Order { price: 200, discount: 15 };
    unsafe {
        let total = order.total();
        println!("Total: {}", total);
    }
}