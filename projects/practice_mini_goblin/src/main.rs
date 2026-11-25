#[derive(Debug)]
struct Character {
    name: String,
    health: u32,
    max_health: u32,
    level: u32,
}

impl Character {
    fn new(name: &str) -> Character {
        Character {
            name: name.to_string(),
            health: 100,
            max_health: 100,
            level: 1,
        }
    }

    fn take_damage(&mut self, ammount: u32) {
        self.health = self.health.saturating_sub(ammount);
    }

    fn heal(&mut self, ammount: u32) {
        let mola: u32 = self.health + ammount;
        if mola < self.max_health {
            self.health = mola;
        }
        else {
            self.health = self.max_health;
        }
    }

    fn level_up(&mut self) {
        self.level += 1;
        self.max_health += 20;
        self.health = self.max_health;
    }

    fn print_stats(&self) {
        println!(
            "Hero: {n} (Level {l}) Health: {h}/{mh}.",
            n = self.name,
            l = self.level,
            h = self.health,
            mh = self.max_health,
        );
    }
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