use core::panic;
use std::{io::{self}};
use rand::Rng;
use colored::*;

enum Zones {
    Desert,
    Forest,
    Mountain,
    Underground,
    Space,
}


fn main() {
    // Set character name
    let name = set_name();
    println!("{} {} {}", "hello".green(), name.green(), "!".green());

    // create inventory and add starting items
    let inventory = create_inventory();    
    let inventory = add_item_to_inventory(inventory, "backpack");

    println!("{}", "You awaken in a strange land, you feel around and discover a few items in your pockets: ".yellow());
    for item in &inventory {
        println!("{} {}", "-".yellow(),  item.yellow())
    }

    // let zone = biome_picker();

    loop {

        let zone = biome_picker();
        if zone == "desert" {
            adventure(Zones::Desert);
        }
        else if zone == "forest" {
            adventure(Zones::Forest);
        }
        else if zone == "mountain" {
            adventure(Zones::Mountain);
        }
        else if zone == "underground" {
            adventure(Zones::Underground);
        }
        else if zone == "space" {
            adventure(Zones::Space);
        }
        else {
            panic!()
        }
    }


}

fn set_name() -> String {
    let mut guess = String::new();

    println!("{}", "Please enter your character name:".green());

    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");

    let trimmed_name: String = (*guess.trim()).to_string();
    
    trimmed_name
}

fn input() -> String {
    let mut my_input = String::new();
    io::stdin()
        .read_line(&mut my_input)
        .expect("failed to read input");
    my_input
}

fn create_inventory() -> Vec<String>{
    // Create empty inventory
    let mut inventory: Vec<String> = Vec::new();

    // add starting items
    inventory.push("knife".to_string());
    inventory.push("Rope".to_string());

    inventory
}

fn add_item_to_inventory(mut my_inventory: Vec<String>, new_item: &str) -> Vec<String>  {
    let item = new_item.to_string();
    my_inventory.push(item);
    my_inventory
}

fn biome_picker() -> String{
    let mut biome: Vec<String> = Vec::new();

    biome.push("desert".to_string());
    biome.push("forest".to_string());
    biome.push("mountain".to_string());
    biome.push("underground".to_string());
    biome.push("space".to_string());

    let roll = rand::thread_rng().gen_range(0..biome.len());

    let my_zone = biome[roll].clone();

    my_zone

}

fn adventure(area: Zones) {
    match area {
        Zones::Desert => {
            desert()
        },
        Zones::Forest => {
            forest()
        },
        Zones::Mountain => {
            mountain()
        },
        Zones::Underground => {
            underground()
        },
        Zones::Space => {
            space()
        },
    }
}

fn desert() {
    println!("{}", "you are in a desert, What do you do?".green());
    let action = input();
    println!("You perform {action}");
}

fn forest() {
    println!("{}", "you are in a Forest, What do you do?".green());
    let action = input();
    println!("You perform {action}");
}

fn mountain() {
    println!("{}", "you are in a Mountain, What do you do?".green());
    let action = input();
    println!("You perform {action}");
}

fn underground() {
    println!("{}", "you are in a Underground, What do you do?".green());
    let action = input();
    println!("You perform {action}");
}

fn space() {
    println!("{}", "you are in a Space, What do you do?".green());
    let action = input();
    println!("You perform {action}");
}