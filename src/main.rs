/*
* --- CHEST LOOTING GAME ---
* This is a chest looting game in which the player rolls
* a dice and selects a chest, leading to different
* outcomes.
*/

/*
* # TODO
* - [x] Keys and locked chests.
* - [ ] Game loop.
* - [ ] Player status reports.
* - [ ] Game over.
*/

use crate::{Critical::*, Loot::*};

const TOTAL_CHESTS: u8 = 100;

fn main() {
    println!("WELCOME TO THE CHEST LOOTING GAME!");
    let mut player = Player::new();
    let player_dice = Dice::roll();
    println!("You rolled a dice: {}", player_dice.0);
    let critical_status = player_dice.is_critical();
    match critical_status {
        Hit => println!("Oh yes!"),
        Miss => println!("Oh no!"),
        None => (),
    }
    let chests = [Chest::new(), Chest::new(), Chest::new()];
    let user_selection = get_user_selection();
    player.open_chest(player_dice, critical_status, chests, user_selection);
}

#[derive(Debug)]
struct Player {
    hp: i8,
    gold: usize,
    keys: u8,
    weapon_dmg: usize,
    remaining_chests: u8,
}

impl Player {
    fn new() -> Self {
        Self {
            hp: 100,
            gold: 0,
            keys: 3,
            weapon_dmg: 10,
            remaining_chests: TOTAL_CHESTS,
        }
    }
    fn open_chest(
        &mut self,
        player_dice: Dice,
        critical_status: Critical,
        chests: [Chest; 3],
        user_selection: usize,
    ) {
        let user_selection = user_selection - 1;
        self.remaining_chests -= 1;
        if chests[user_selection].locked {
            if self.keys < 1 {
                println!("This chest was locked but you had no keys left!");
                return;
            }
            println!("You used a key to open this chest!");
            self.keys -= 1;
        }
        let weapon_name = match rand::random_range(0..7) {
            0 => "a Sword",
            1 => "an Axe",
            2 => "a Bow",
            3 => "a Dagger",
            4 => "a Spear",
            5 => "a Mace",
            _ => "a Hammer",
        };
        match critical_status {
            Hit => {
                match chests[user_selection].loot {
                    Potion => {
                        self.hp += 30;
                        if self.hp > 100 {
                            self.hp = 100;
                        }
                        println!("You found a potion! You recovered 30 HP.")
                    }
                    Gold(g) => {
                        self.gold += g * 2;
                        println!("You found {} gold!", g * 2);
                    }
                    Key => {
                        self.keys += 2;
                        println!("You found not one, but two keys!");
                    }
                    Weapon { damage: d } => {
                        if self.weapon_dmg < d * 2 {
                            self.weapon_dmg = d * 2;
                            println!(
                                "You found {}! Your new weapon does {} damage.",
                                weapon_name,
                                d * 2
                            );
                        } else {
                            println!(
                                "You found {}, but your current weapon is stronger.",
                                weapon_name
                            );
                        };
                    }
                    Nothing => {
                        self.gold += 50;
                        println!(
                            "The chest was empty, but your infinite luck\n\
                            made you find 50 gold on the floor anyway!"
                        )
                    }
                };
            }
            Miss => {
                self.hp -= 20;
            }
            None => {
                if player_dice.0 > chests[user_selection].dice.0 {
                    match chests[user_selection].loot {
                        Potion => {
                            self.hp += 15;
                            if self.hp > 100 {
                                self.hp = 100;
                            };
                            println!("You found a potion! You recovered 15 HP.");
                        }
                        Gold(g) => {
                            self.gold += g;
                            println!("You found {} gold!", g);
                        }
                        Key => {
                            self.keys += 1;
                            println!("You found a key!");
                        }
                        Weapon { damage: d } => {
                            if self.weapon_dmg < d {
                                self.weapon_dmg = d;
                                println!(
                                    "You found {}! Your new weapon does {} damage.",
                                    weapon_name, d
                                );
                            } else {
                                println!(
                                    "You found {}, but your current weapon is stronger.",
                                    weapon_name
                                );
                            };
                        }
                        Nothing => println!("Bad luck! The chest was empty!"),
                    };
                } else {
                    println!("You were too clumsy to open this chest!");
                }
            }
        }
    }
}

#[derive(Debug)]
enum Critical {
    Hit,
    Miss,
    None,
}

#[derive(Debug)]
struct Dice(u8);

impl Dice {
    fn roll() -> Self {
        Self(rand::random_range(1..=20))
    }
    fn is_critical(&self) -> Critical {
        match self.0 {
            20 => Critical::Hit,
            1 => Critical::Miss,
            _ => Critical::None,
        }
    }
}

#[derive(Debug)]
enum Loot {
    Potion,
    Gold(usize),
    Key,
    Weapon { damage: usize },
    Nothing,
}

impl Loot {
    fn new() -> Self {
        match rand::random_range(0..5) {
            0 => Self::Potion,
            1 => Self::Gold(rand::random_range(20..=200)),
            2 => Self::Key,
            3 => Self::Weapon {
                damage: rand::random_range(1..=4999),
            },
            _ => Self::Nothing,
        }
    }
}

#[derive(Debug)]
struct Chest {
    loot: Loot,
    dice: Dice,
    locked: bool,
}

impl Chest {
    fn new() -> Self {
        Self {
            loot: Loot::new(),
            dice: Dice::roll(),
            locked: match rand::random_range(0..2) {
                0 => false,
                _ => true,
            },
        }
    }
}

fn get_user_selection() -> usize {
    const ERROR_MSG: &str = "Please type a number from 1 to 3!";
    println!(
        "\n\
        Which chest do you want to open?\n\
        \n\
        1. Chest 1.\n\
        2. Chest 2.\n\
        3. Chest 3.\n"
    );
    let mut input = String::new();
    loop {
        input.clear();
        if let Err(error) = std::io::stdin().read_line(&mut input) {
            println!("Failed to get user input with error:\n{}", error);
            continue;
        }
        match input.trim().parse() {
            Err(_) => {
                println!("{}", ERROR_MSG);
                continue;
            }
            Ok(selection) => {
                if selection < 1 || selection > 3 {
                    println!("{}", ERROR_MSG);
                    continue;
                }
                return selection;
            }
        }
    }
}
