/*
* --- CHEST LOOTING GAME ---
* This is a chest looting game in which the player rolls
* a dice and selects a chest, leading to different
* outcomes.
*/

use crate::Critical::*;

const TOTAL_CHESTS: u8 = 100;

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
    Potion { hp_regen: u8 },
    Gold(usize),
    Key { number: u8 },
    Weapon { damage: usize },
    Nothing,
}

impl Loot {
    fn new() -> Self {
        match rand::random_range(0..5) {
            0 => Self::Potion {
                hp_regen: rand::random_range(10..=50),
            },
            1 => Self::Gold(rand::random_range(20..=200)),
            2 => Self::Key { number: 1 },
            3 => Self::Weapon {
                damage: rand::random_range(1..=9999),
            },
            _ => Self::Nothing,
        }
    }
}

#[derive(Debug)]
struct Chest {
    loot: Loot,
    dice: Dice,
}

impl Chest {
    fn new() -> Self {
        Self {
            loot: Loot::new(),
            dice: Dice::roll(),
        }
    }
}

fn get_user_selection() -> u8 {
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

fn main() {
    println!("WELCOME TO THE CHEST LOOTING GAME!");
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
}

/*
* # TASKLIST
* - [x] Player rolls a dice.
* - [x] Three chests with loot and dices are generated.
* - [x] The player chooses a chest.
* - [x] The player gets special outcome on:
*   - Natural 20.
*   - Natural 1.
* - [/] The game checks the duel for a:
*   - Regular Success.
*   - Regular Miss.
* (...)
*/
