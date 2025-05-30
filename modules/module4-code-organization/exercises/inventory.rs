use crate::product::Product;
use std::collections::HashMap;

pub struct Inventory {
    products: HashMap<u32, (Product, u32)>, // (Product, quantity)
}

impl Inventory {
    pub fn new() -> Self {
        Inventory {
            products: HashMap::new(),
        }
    }

    pub fn add_product(&mut self, product: Product, quantity: u32) {
        self.products.insert(product.id, (product, quantity));
    }

    pub fn remove_product(&mut self, product_id: u32, quantity: u32) -> bool {
        if let Some((_, current_quantity)) = self.products.get_mut(&product_id) {
            if *current_quantity >= quantity {
                *current_quantity -= quantity;
                return true;
            }
        }
        false
    }

    pub fn get_stock(&self, product_id: u32) -> Option<u32> {
        self.products.get(&product_id).map(|(_, quantity)| *quantity)
    }

    pub fn display(&self) {
        println!("Inventory Status:");
        for (_, (product, quantity)) in &self.products {
            println!("{}: {} units in stock", product.name, quantity);
        }
    }
}