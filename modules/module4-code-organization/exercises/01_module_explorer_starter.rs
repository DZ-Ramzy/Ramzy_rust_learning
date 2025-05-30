mod product;
mod user;
mod order;
mod inventory;

use product::Product;
use user::User;
use order::{Order, OrderStatus};
use inventory::Inventory;

fn main() {
    // Create some products
    let laptop = Product::new(
        1,
        "Laptop".to_string(),
        999.99,
        "High-performance laptop".to_string(),
    );
    let mouse = Product::new(
        2,
        "Mouse".to_string(),
        29.99,
        "Wireless mouse".to_string(),
    );

    // Create inventory and add products
    let mut inventory = Inventory::new();
    inventory.add_product(laptop.clone(), 10);
    inventory.add_product(mouse.clone(), 50);

    // Create a user
    let user = User::new(
        1,
        "John Doe".to_string(),
        "john@example.com".to_string(),
        "123 Main St".to_string(),
    );

    // Create an order
    let mut order = Order::new(1, user.clone(), vec![laptop, mouse]);

    // Display everything
    println!("\n=== User Information ===");
    user.display();

    println!("\n=== Inventory Status ===");
    inventory.display();

    println!("\n=== Order Details ===");
    order.display();
}