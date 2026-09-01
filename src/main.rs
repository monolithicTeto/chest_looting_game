/*
* --- CHEST LOOTING GAME ---
* This is a chest looting game in which the player rolls
* a dice and selects a chest, leading to different
* outcomes.
*/

/* TODO
* - [ ] Move system messages to constants.
* - [ ] Add an exit selection.
* - [ ] Clear the terminal on each loop.
*/

use crate::{Critical::*, Loot::*};
use std::process::exit;

const TOTAL_CHESTS: i8 = 100;

fn main() {
    println!("\nWELCOME TO THE CHEST LOOTING GAME!");
    let mut player = Player::new();
    while player.remaining_chests > 0 {
        let player_dice = Dice::roll();
        println!(
            "There are {} chests left in the dungeon.\n\
            You rolled a dice: {}",
            player.remaining_chests, player_dice.0
        );
        let critical_status = player_dice.is_critical();
        match critical_status {
            Hit => println!("Oh yes!"),
            Miss => println!("Oh no!"),
            None => (),
        }
        let chests = [Chest::new(), Chest::new(), Chest::new()];
        let user_selection = get_user_selection();
        player.open_chest(player_dice, critical_status, chests, user_selection);
        if player.hp < 1 {
            println!(
                "Your wounds made you bleed to death.\n\
                \n\
                Thanks for playing!"
            );
            exit(0)
        }
    }
    println!(
        "You cleared all the chests!\n\
        \n\
        You walked out of the dungeon with:\n\
        {} HP left.\n\
        {} gold.\n\
        A powerful weapon that deals {} points of damage.\n\
        \n\
        Thanks for playing!",
        player.hp, player.gold, player.weapon_dmg
    );
    exit(0)
}

#[derive(Debug)]
struct Player {
    hp: i8,
    gold: usize,
    keys: u8,
    weapon_dmg: usize,
    remaining_chests: i8,
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
        self.remaining_chests -= 3;
        if chests[user_selection].locked {
            if self.keys < 1 {
                println!("\nThis chest was locked but you had no keys left!\n");
                return;
            }
            self.keys -= 1;
            println!(
                "\nYou used a key to open this chest!\n\
                Keys left: {}",
                self.keys
            );
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
                        println!("\nYou found a potion! You recovered 30 HP.\n")
                    }
                    Gold(g) => {
                        self.gold += g * 2;
                        println!(
                            "\nYou found {} gold!\n\
                            You now have {} gold.\n",
                            g * 2,
                            self.gold
                        );
                    }
                    Key => {
                        self.keys += 2;
                        println!(
                            "\nYou found not one, but two keys!\n\
                            You now have {} keys.\n",
                            self.keys
                        );
                    }
                    Weapon { damage: d } => {
                        if self.weapon_dmg < d * 2 {
                            self.weapon_dmg = d * 2;
                            println!(
                                "\nYou found {}! Your new weapon does {} damage.\n",
                                weapon_name,
                                d * 2
                            );
                        } else {
                            println!(
                                "\nYou found {}, but your current weapon is stronger.\n",
                                weapon_name
                            );
                        };
                    }
                    Nothing => {
                        self.gold += 50;
                        println!(
                            "\nThe chest was empty, but your infinite luck\n\
                            made you find 50 gold on the floor anyway!\n"
                        )
                    }
                };
            }
            Miss => {
                self.hp -= 20;
                println!("\nThe chest bit you back! You took 20 points of damage.\n");
            }
            None => {
                if player_dice.0 > chests[user_selection].dice.0 {
                    match chests[user_selection].loot {
                        Potion => {
                            self.hp += 15;
                            if self.hp > 100 {
                                self.hp = 100;
                            };
                            println!("\nYou found a potion! You recovered 15 HP.\n");
                        }
                        Gold(g) => {
                            self.gold += g;
                            println!(
                                "\nYou found {} gold!\n\
                                You now have {} gold.\n",
                                g, self.gold
                            );
                        }
                        Key => {
                            self.keys += 1;
                            println!(
                                "\nYou found a key!\n\
                                You now have {} keys.\n",
                                self.keys
                            );
                        }
                        Weapon { damage: d } => {
                            if self.weapon_dmg < d {
                                self.weapon_dmg = d;
                                println!(
                                    "\nYou found {}! Your new weapon does {} damage.\n",
                                    weapon_name, d
                                );
                            } else {
                                println!(
                                    "\nYou found {}, but your current weapon is stronger.\n",
                                    weapon_name
                                );
                            };
                        }
                        Nothing => println!("\nBad luck! The chest was empty!\n"),
                    };
                } else {
                    println!("\nYou were too clumsy to open this chest!\n");
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
