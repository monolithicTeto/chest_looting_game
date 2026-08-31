/*
* --- CHEST LOOT GAME ---
* This is a chest loot game in which the player rolls
* a dice and selects a chest, leading to different
* outcomes.
*/

#[derive(Debug)]
struct Dice(u8);

impl Dice {
    fn roll() -> Self {
        Self(rand::random_range(1..=20))
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

fn main() {
    let player_dice = Dice::roll();
    let chests = [Chest::new(), Chest::new(), Chest::new()];
    dbg!(&player_dice);
    dbg!(&chests);
}
