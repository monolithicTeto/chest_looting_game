/* --- CHEST LOOTING GAME ---
* This is a chest looting game in which the player rolls
* a die and selects a chest, leading to different
* outcomes.
*/

// Reduces cross-terminal stylization and manipulation to one library.
use crossterm::{
    cursor::MoveTo,
    style::{Color, Stylize},
    terminal::{Clear, ClearType},
};

const SHOW_DEBUG: bool = false;
const TOTAL_CHESTS: i8 = 33 * 3;
const TEXT_COLORS: TextColors = TextColors {
    green: [95, 169, 104],  // #5FA968
    blue: [61, 125, 191],   // #3D7DBF
    yellow: [204, 155, 60], // #CC9B3C
    orange: [217, 122, 61], // #D97A3D
    red: [209, 71, 111],    // #D1476F
};

fn main() {
    'retry_loop: loop {
        clear_terminal();
        if SHOW_DEBUG == false {
            println!(
                "\nWELCOME TO THE CHEST LOOTING GAME!\n\
                ----------------------------------\n\
                Armed with a Rusty Sword and your courage,\n\
                you delve into the depths of a dungeon in\n\
                search of unfathomable fortunes.\n"
            );
        };
        let mut player = Player::new();
        while player.remaining_chests > 0 {
            if SHOW_DEBUG {
                println!("------------------------------------");
                dbg!(&player);
                println!("------------------------------------\n");
            }
            let player_die = Die::roll(false);
            println!(
                "There are {} chests left in the dungeon.\n\
                You rolled a die: {}",
                player.remaining_chests, player_die.0
            );
            let critical_status = player_die.is_critical();
            match critical_status {
                Critical::Hit => println!("Oh yes!"),
                Critical::Miss => println!("Oh no!"),
                Critical::None => (),
            }
            if player.hp < 15 {
                let [r, g, b] = TEXT_COLORS.red;
                println!(
                    "\n{}",
                    "You can feel death closing in..."
                        .with(Color::Rgb { r: r, g: g, b: b })
                        .bold()
                );
            } else if player.hp < 33 {
                let [r, g, b] = TEXT_COLORS.orange;
                println!(
                    "\n{}",
                    "You're losing consciousness..."
                        .with(Color::Rgb { r: r, g: g, b: b })
                        .bold()
                );
            } else if player.hp < 66 {
                let [r, g, b] = TEXT_COLORS.yellow;
                println!(
                    "\n{}",
                    "You feel a little dizzy..."
                        .with(Color::Rgb { r: r, g: g, b: b })
                        .bold()
                );
            }
            let chests = [Chest::new(), Chest::new(), Chest::new()];
            if SHOW_DEBUG {
                println!("\n------------------------------------");
                dbg!(&chests);
                println!("------------------------------------");
            }
            println!(
                "\n\
                Which chest do you want to open?\n\
                \n\
                1. A {}-looking Chest.\n\
                2. A {}-looking Chest.\n\
                3. A {}-looking Chest.\n\
                \n\
                0. Leave the dungeon.\n",
                chests[0].is_strong(),
                chests[1].is_strong(),
                chests[2].is_strong()
            );
            let user_selection = get_user_selection(3);
            clear_terminal();
            if user_selection == 0 {
                player.print_exit();
                if retry_prompt() == 1 {
                    continue 'retry_loop;
                };
                std::process::exit(0);
            }
            player.open_chest(player_die, critical_status, chests, user_selection);
            if player.hp < 1 {
                println!(
                    "You bleed to death from your wounds.\n\
                    \n\
                    Your corpse lies somewhere in the depths\n\
                    of a dungeon, never to be found again.\n\
                    \n\
                    Thanks for playing!\n"
                );
                if retry_prompt() == 1 {
                    continue 'retry_loop;
                };
                std::process::exit(0);
            }
            if player.remaining_chests > 0 {
                println!("You walk over to the next room.\n");
            };
        }
        println!("You cleared all the chests!\n");
        player.print_exit();
        if retry_prompt() == 1 {
            continue 'retry_loop;
        };
        std::process::exit(0);
    }
}

#[allow(dead_code)]
struct TextColors {
    green: [u8; 3],
    blue: [u8; 3],
    yellow: [u8; 3],
    orange: [u8; 3],
    red: [u8; 3],
}

#[derive(Debug)]
struct Player {
    hp: i16,
    gold: usize,
    keys: u8,
    weapon_dmg: usize,
    remaining_chests: i8,
}

impl Player {
    fn new() -> Self {
        Self {
            hp: 100,
            gold: 15,
            keys: 3,
            weapon_dmg: 10,
            remaining_chests: TOTAL_CHESTS,
        }
    }
    fn open_chest(
        &mut self,
        player_die: Die,
        critical_status: Critical,
        chests: [Chest; 3],
        user_selection: usize,
    ) {
        let user_selection = user_selection - 1;
        self.remaining_chests -= 3;
        match critical_status {
            Critical::Miss => (),
            _ => {
                if chests[user_selection].locked {
                    if self.keys < 1 {
                        println!("\nThe chest was locked but you had no keys left!\n");
                        self.monster_attack();
                        return;
                    }
                    println!(
                        "\nThe chest is locked!\n\
                        Keys left: {}\n\
                        \n\
                        Do you want to use a key?\n\
                         1. yes           0. no\n",
                        self.keys
                    );
                    if get_user_selection(1) == 1 {
                        clear_terminal();
                        self.keys -= 1;
                    } else {
                        clear_terminal();
                        self.monster_attack();
                        return;
                    };
                };
            }
        };
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
            Critical::Hit => {
                match chests[user_selection].loot {
                    Loot::Potion => {
                        let p = 30;
                        self.hp += p;
                        if self.hp > 100 {
                            self.hp = 100;
                        }
                        println!("\nYou found a potion! You recovered {} HP.\n", p);
                    }
                    Loot::Gold(g) => {
                        let g = apply_bonuses(g, &chests[user_selection].die) * 2;
                        self.gold += g;
                        println!(
                            "\nYou found {} gold!\n\
                            You now have {} gold.\n",
                            g, self.gold
                        );
                    }
                    Loot::Key => {
                        self.keys += 2;
                        println!(
                            "\nYou found not one, but two keys!\n\
                            You now have {} keys.\n",
                            self.keys
                        );
                    }
                    Loot::Weapon { damage: d } => {
                        let d = apply_bonuses(d, &chests[user_selection].die) * 2;
                        let d = if d > 9999 { 9999 } else { d };
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
                    Loot::Nothing => {
                        let g = rand::random_range(40..=70);
                        self.gold += g;
                        println!(
                            "\nThe chest was empty, but your infinite luck\n\
                            made you find {} gold on the floor anyway!\n",
                            g
                        )
                    }
                };
            }
            Critical::Miss => {
                let d = rand::random_range(17..=23);
                self.hp -= d;
                println!(
                    "\nThe chest bit back at your hand! You took {} points of damage.\n",
                    d
                );
            }
            Critical::None => {
                if player_die.0 >= chests[user_selection].die.0 {
                    match chests[user_selection].loot {
                        Loot::Potion => {
                            let p = 15;
                            self.hp += p;
                            if self.hp > 100 {
                                self.hp = 100;
                            };
                            println!("\nYou found a potion! You recovered {} HP.\n", p);
                        }
                        Loot::Gold(g) => {
                            let g = apply_bonuses(g, &chests[user_selection].die);
                            self.gold += g;
                            println!(
                                "\nYou found {} gold!\n\
                                You now have {} gold.\n",
                                g, self.gold
                            );
                        }
                        Loot::Key => {
                            self.keys += 1;
                            println!(
                                "\nYou found a key!\n\
                                You now have {} keys.\n",
                                self.keys
                            );
                        }
                        Loot::Weapon { damage: d } => {
                            let d = apply_bonuses(d, &chests[user_selection].die);
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
                        Loot::Nothing => println!("\nBad luck! The chest was empty!\n"),
                    };
                } else {
                    println!("\nYou were too clumsy to open this chest!\n");
                    self.monster_attack();
                }
            }
        }
    }
    fn print_exit(&self) {
        println!(
            "You walked out of the dungeon with:\n\
            {} HP left.\n\
            {} gold.\n\
            {} weapon that deals {} points of damage.\n\
            \n\
            Thanks for playing!\n",
            self.hp,
            self.gold,
            if self.weapon_dmg < 1000 {
                "A pitiful"
            } else if self.weapon_dmg < 6000 {
                "A powerful"
            } else {
                "An insane"
            },
            self.weapon_dmg
        );
    }
    fn monster_attack(&mut self) {
        println!(
            "You turn around to leave the chest behind,\n\
            but a {} jumps you on your way out!\n",
            match rand::random_range(0..7) {
                0 => "Sneaky Goblin",
                1 => "Cadaver, hmmm,",
                2 => "Sticky Slime",
                3 => "Giant Spider",
                4 => "Cave Crawler",
                5 => "Nightstalker",
                _ => "Dungeon Rat",
            }
        );
        let monster_attack = rand::random_range(1..=8000);
        if SHOW_DEBUG {
            println!("------------------------------------");
            dbg!(&monster_attack);
            println!("------------------------------------\n");
        }
        if self.weapon_dmg > monster_attack {
            let battle_reward = rand::random_range(100..=300);
            if self.weapon_dmg - monster_attack < 200 {
                println!(
                    "Your weapon was barely enough to outpower your foe!\n\
                    The monster dropped a pouch containing {} gold inside.\n\
                    You take no damage from this encounter.\n",
                    battle_reward
                );
            } else if self.weapon_dmg - monster_attack < 2000 {
                println!(
                    "Your weapon proved very effective against your foe!\n\
                    The monster dropped a pouch containing {} gold inside.\n\
                    You take no damage from this encounter.\n",
                    battle_reward
                );
            } else if self.weapon_dmg - monster_attack < 6000 {
                println!(
                    "Your weapon let you defeat your foe effortlessly!\n\
                    The monster dropped a pouch containing {} gold inside.\n\
                    You take no damage from this encounter.\n",
                    battle_reward
                );
            } else {
                println!(
                    "Your weapon obliterated that poor bastard!\n\
                    The monster dropped a pouch containing {} gold inside.\n",
                    battle_reward
                );
            }
            self.gold += battle_reward;
        } else {
            let battle_damage = rand::random_range(15..=30);
            if monster_attack - self.weapon_dmg < 200 {
                println!(
                    "It was a tight fight, but your weapon failed you at last!\n\
                    You managed to flee, but took {} points of damage.\n",
                    battle_damage
                );
            } else if monster_attack - self.weapon_dmg < 2000 {
                println!(
                    "Your weapon did not suffice against your foe!\n\
                    You managed to flee, but took {} points of damage.\n",
                    battle_damage
                );
            } else if monster_attack - self.weapon_dmg < 6000 {
                println!(
                    "Your weapon didn't leave a single scratch on your foe!\n\
                    You managed to flee, but took {} points of damage.\n",
                    battle_damage
                )
            } else {
                println!(
                    "Your weapon didn't do crap and you got smoked in combat!\n\
                    You managed to flee, but took {} points of damage.\n",
                    battle_damage
                )
            };
            if self.hp > 14 && self.hp - battle_damage < 1 {
                self.hp = 1;
            } else {
                self.hp -= battle_damage;
            };
        };
    }
}

#[derive(Debug)]
enum Critical {
    Hit,
    Miss,
    None,
}

#[derive(Debug)]
struct Die(u8);

impl Die {
    fn roll(cap: bool) -> Self {
        if cap {
            Self(rand::random_range(1..=18))
        } else {
            Self(rand::random_range(1..=20))
        }
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
        match rand::random_range(0..100) {
            /* Chance of Nothing:   10%.
             * Chance of Key:       10%.
             * Chance of Potion:    20%.
             * Chance of Gold:      25%.
             * Chance of Weapon:    35%.
             */
            00..10 => Self::Key,
            10..30 => Self::Potion,
            30..55 => Self::Gold(rand::random_range(20..=200)),
            55..90 => Self::Weapon {
                damage: rand::random_range(1..=3334),
            },
            _ => Self::Nothing,
        }
    }
}

#[derive(Debug)]
struct Chest {
    loot: Loot,
    die: Die,
    locked: bool,
}

impl Chest {
    fn new() -> Self {
        Self {
            loot: Loot::new(),
            die: Die::roll(true),
            locked: match rand::random_range(0..5) {
                0 => true,
                _ => false,
            },
        }
    }
    fn is_strong(&self) -> &str {
        if self.die.0 < rand::random_range(5..=7) {
            "Flimsy"
        } else if self.die.0 < rand::random_range(13..=15) {
            "Regular"
        } else {
            "Sturdy"
        }
    }
}

fn get_user_selection(selection_range: usize) -> usize {
    let error_msg: String = format!("\nPlease type a number from 0 to {}!\n", selection_range);
    let mut input = String::new();
    loop {
        input.clear();
        std::io::stdin()
            .read_line(&mut input)
            .expect("ERROR: failed to get user input.");
        let Ok(selection) = input.trim().parse() else {
            println!("{}", error_msg);
            continue;
        };
        if selection > selection_range {
            println!("{}", error_msg);
            continue;
        }
        return selection;
    }
}

fn retry_prompt() -> usize {
    let [r, g, b] = TEXT_COLORS.blue;
    println!(
        "{}",
        "Do you want to start again?\n\
        \n\
        1. Start again!\n\
        0. Quit the game.\n"
            .with(Color::Rgb { r: r, g: g, b: b })
            .bold()
    );
    get_user_selection(1)
}

fn apply_bonuses(n: usize, d: &Die) -> usize {
    if d.0 < 7 {
        n / 2
    } else if d.0 < 15 {
        n
    } else {
        n / 2 * 3
    }
}

fn clear_terminal() {
    if SHOW_DEBUG == false {
        crossterm::execute!(std::io::stdout(), Clear(ClearType::All), MoveTo(0, 0))
            .expect("ERROR: failed to clear terminal.")
    };
}
