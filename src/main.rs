/* --- CHEST LOOTING GAME ---
* This is a chest looting game in which the player rolls
* a dice and selects a chest, leading to different
* outcomes.
*/

/* TODO
* - [x] Clear the terminal on each loop.
* - [x] Lower the likelyhood of chests being locked to 20%.
* - [x] Add an exit selection.
* - [x] Add hints to how difficult each chest is to open.
*   - Sturdy-looking.
*   - Regular-looking.
*   - Flimsy-looking.
* - [x] Add bonuses depending on if the chest's dice is:
*   - More than 14.
*   - Between 6 and 14.
*   - Less than 6.
* - [x] Make it possible to refuse spending a key.
*   - [x] Add monster attacks when a locked chest is not opened.
*       - Flavored text get printed when the damage difference is:
*           - More than 6000 positive.
*           - More than 2000 positive.
*           - More than 200 positive.
*           - More than 200 negative.
*           - More than 2000 negative.
*           - More than 6000 negative.
*       - On loses, the player takes between 15 and 30 damage.
* - [/] Implement the "Director."
*     - [ ] Silently grant a d20 after a number of misses.
*     - [ ] Silently grant a d1 after a number of hits.
* - [~] Make it so succesfully opening a locked chest never gets you Nothing.
* - [~] Move system messages to constants.
* - [~] More flavor text to make the game more interesting.
*/

use std::process::exit;

const TOTAL_CHESTS: i8 = 33 * 3;
const SHOW_DEBUG: bool = true;

fn main() {
    clear_terminal();
    println!(
        "\nWELCOME TO THE CHEST LOOTING GAME!\n\
        ----------------------------------\
        \n"
    );
    let mut player = Player::new();
    if SHOW_DEBUG {
        dbg!(&player);
        println!("------------------------------------\n");
    }
    while player.remaining_chests > 0 {
        let player_dice = Dice::roll(false);
        println!(
            "There are {} chests left in the dungeon.\n\
            You rolled a dice: {}",
            player.remaining_chests, player_dice.0
        );
        let critical_status = player_dice.is_critical();
        match critical_status {
            Critical::Hit => println!("Oh yes!"),
            Critical::Miss => println!("Oh no!"),
            Critical::None => (),
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
            0. Leave.\n",
            chests[0].is_strong(),
            chests[1].is_strong(),
            chests[2].is_strong()
        );
        let user_selection = get_user_selection(3);
        clear_terminal();
        if user_selection == 0 {
            player.print_exit();
            exit(0);
        }
        player.open_chest(player_dice, critical_status, chests, user_selection);
        if player.hp < 1 {
            println!(
                "Your wounds made you bleed to death.\n\
                \n\
                Thanks for playing!\n"
            );
            exit(0)
        }
    }
    println!("You cleared all the chests!\n");
    player.print_exit();
    exit(0)
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
        player_dice: Dice,
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
                        self.keys -= 1;
                    } else {
                        println!(
                            "You turn around to leave the chest behind,\n\
                            but a {} jumps you on your way out!\n",
                            match rand::random_range(0..7) {
                                0 => "Sneaky Goblin",
                                1 => "Walking Skeleton",
                                2 => "Sticky Slime",
                                3 => "Giant Spider",
                                4 => "Cave Crawler",
                                5 => "Nightstalker",
                                _ => "Dungeon Rat",
                            }
                        );
                        let monster_attack = rand::random_range(1..=8000);
                        if self.weapon_dmg > monster_attack {
                            if self.weapon_dmg - monster_attack < 200 {
                                println!(
                                    "Your weapon was barely enough to outpower your foe!\n\
                                    You take no damage from this encounter.\n"
                                );
                            } else if self.weapon_dmg - monster_attack < 2000 {
                                println!(
                                    "Your weapon proved very effective against your foe!\n\
                                    You take no damage from this encounter.\n"
                                );
                            } else if self.weapon_dmg - monster_attack < 6000 {
                                println!(
                                    "Your weapon had you defeat your foe effortlessly!\n\
                                    You take no damage from this encounter.\n"
                                );
                            } else {
                                println!("Your weapon obliterated that poor bastard!\n");
                            }
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
                                    "Your weapon didn't do your foe a single scratch!\n\
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
                            self.hp -= battle_damage;
                        };
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
                        let g = apply_bonuses(g, &chests[user_selection].dice) * 2;
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
                        let d = apply_bonuses(d, &chests[user_selection].dice) * 2;
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
                if player_dice.0 >= chests[user_selection].dice.0 {
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
                            let g = apply_bonuses(g, &chests[user_selection].dice);
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
                            let d = apply_bonuses(d, &chests[user_selection].dice);
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
                }
            }
        }
        if SHOW_DEBUG {
            println!("----------------------------------------");
            dbg!(&self);
            println!("----------------------------------------\n");
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
        match rand::random_range(0..1000) {
            /* Chance of Nothing: 10%.
             * Anything else: 22.5%.
             */
            000..225 => Self::Potion,
            225..450 => Self::Gold(rand::random_range(20..=200)),
            450..675 => Self::Key,
            675..900 => Self::Weapon {
                damage: rand::random_range(1..=3334),
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
            dice: Dice::roll(true),
            locked: match rand::random_range(0..5) {
                0 => true,
                _ => false,
            },
        }
    }
    fn is_strong(&self) -> &str {
        if self.dice.0 < rand::random_range(5..=7) {
            "Flimsy"
        } else if self.dice.0 < rand::random_range(13..=15) {
            "Regular"
        } else {
            "Sturdy"
        }
    }
}

fn get_user_selection(selection_range: usize) -> usize {
    let error_msg: String = format!("Please type a number from 0 to {}!", selection_range);
    let mut input = String::new();
    loop {
        input.clear();
        if let Err(error) = std::io::stdin().read_line(&mut input) {
            println!("Failed to get user input with error:\n{}", error);
            continue;
        }
        match input.trim().parse() {
            Err(_) => {
                println!("{}", error_msg);
                continue;
            }
            Ok(selection) => {
                if selection > selection_range {
                    println!("{}", error_msg);
                    continue;
                }
                return selection;
            }
        }
    }
}

fn apply_bonuses(n: usize, d: &Dice) -> usize {
    if d.0 < 7 {
        n / 2
    } else if d.0 < 15 {
        n
    } else {
        n / 2 * 3
    }
}

fn clear_terminal() {
    /* 1B: terminal ESC command.
     * [2J: clears the terminal.
     * [1;1H: moves the cursor back to position '1;1'.
     */
    if SHOW_DEBUG == false {
        print!(
            "\x1B[2J\
            \x1B[1;1H"
        );
    };
}
