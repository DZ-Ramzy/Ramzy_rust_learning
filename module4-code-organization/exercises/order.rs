use crate::product::Product;
use crate::user::User;

#[derive(Debug)]
pub enum OrderStatus {
    Pending,
    Processing,
    Shipped,
    Delivered,
}

pub struct Order {
    pub id: u32,
    pub user: User,
    pub products: Vec<Product>,
    pub status: OrderStatus,
    pub total: f64,
}

impl Order {
    pub fn new(id: u32, user: User, products: Vec<Product>) -> Self {
        let total = products.iter().map(|p| p.price).sum();
        Order {
            id,
            user,
            products,
            status: OrderStatus::Pending,
            total,
        }
    }

    pub fn update_status(&mut self, status: OrderStatus) {
        self.status = status;
    }

    pub fn display(&self) {
        println!("Order #{}", self.id);
        println!("Customer: {}", self.user.name);
        println!("Status: {:?}", self.status);
        println!("Products:");
        for product in &self.products {
            println!("- {} (${:.2})", product.name, product.price);
        }
        println!("Total: ${:.2}", self.total);
    }
}