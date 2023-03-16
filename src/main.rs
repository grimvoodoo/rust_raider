use std::io::{self};

fn main() {
    // Set character name
    let name = set_name();
    println!("Hello {name}!");

    // create inventory and add starting items
    let inventory = create_inventory();    
    let inventory = add_item_to_inventory(inventory, "backpack");

    println!("You awaken in a strange land, you feel around and discover a few items in your pockets: ");
    for item in &inventory {
        println!("{item}")
    }


}

fn set_name() -> String {
    let mut guess = String::new();

    println!("Please enter your character name:");

    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");

    let trimmed_name: String = (*guess.trim()).to_string();
    
    trimmed_name
}

fn create_inventory() -> Vec<String>{
    // Create empty inventory
    let mut inventory: Vec<String> = Vec::new();

    // add starting items
    inventory.push("knife".to_string());
    inventory.push("Rope".to_string());

    inventory
}

fn add_item_to_inventory(mut my_inventory: Vec<String>, new_item: &str) -> Vec<String> {
    let item = new_item.to_string();
    my_inventory.push(item);
    my_inventory
}