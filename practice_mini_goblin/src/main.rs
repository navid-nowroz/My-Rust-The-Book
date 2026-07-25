struct Character {
    // Add fields here!
}

impl Character {
    // Add your associated function (new) and methods here!
}

fn main() {
    let mut hero = Character::new("Aria");
    hero.print_stats();

    println!("\n--- A wild goblin attacks! ---");
    hero.take_damage(35);
    hero.print_stats();

    println!("\n--- Drinking a health potion... ---");
    hero.heal(20);
    hero.print_stats();

    println!("\n--- Level Up! ---");
    hero.level_up();
    hero.print_stats();
}