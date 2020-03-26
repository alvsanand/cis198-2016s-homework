use std;
use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;

use super::curio::Curio;
use super::room::Room;

const MAX_HP: i32 = 25;

pub enum Command {
    Go(String),
    Shoot(String),
}

pub struct Player {
    pub location: Rc<RefCell<Room>>,
    pub hp: i32,
    pub gold: i32,
    pub won: bool,
}

impl Player {
    pub fn new(location: Rc<RefCell<Room>>) -> Player {
        Player {
            location: location,
            hp: MAX_HP,
            gold: 0,
            won: false,
        }
    }

    pub fn use_curio(&mut self, curio: Curio) {
        match curio {
            Curio::Chest(gold) => {
                println!("You open the chest and gain {} gold.", gold);
                self.gold += gold;
            },
            Curio::SpikeTrap(dmg) => {
                println!("You take {} damage from the spikes.", dmg);
                self.hp -= dmg;
            },
            Curio::Food(heal) => {
                println!("You shove a wall chicken into your gob and heal {} HP.", heal);
                self.hp = std::cmp::min(MAX_HP, self.hp + heal);
            },
            Curio::IronMaiden(sub, dmg) => {
                println!("Dude I love Iron Maiden! This one's pointy, though.");
                println!("You cut yourself on the spikes inside for {} damage.", dmg);
                self.hp -= dmg;
                println!("You open the iron maiden and...");
                self.use_curio(*sub);
            },
            Curio::FallenAdventurer(sub) => {
                println!("You pilfer the corpse and...");
                self.use_curio(*sub);
            },
        }
    }

    /// Execute the given command on the player and board state.
    pub fn act(&mut self, cmd: Command) -> Result<(), ()> {
        match cmd {
            Command::Go(to) => {
                self.find_room(to).and_then(|room|{
                    let curios: Vec<Curio> = (*(room.borrow()).contents).to_vec();
                    for curio in curios {
                        self.use_curio(curio);
                    }                    
                    
                    self.location = room;

                    Ok(())
                })
            }
            Command::Shoot(to) => {
                self.find_room(to).and_then(|room| {
                    if room.borrow().wumpus {
                        self.won = true;
                    }

                    Ok(())
                })
            }
        }
    }

    /// Find one of the neighbors of the current room based on its name. Case insensitive.
    fn find_room(&self, room: String) -> Result<Rc<RefCell<Room>>, ()> {
        self.location.borrow().find_room(room)
    }
}

/**/impl fmt::Display for Player {
/**/    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
/**/        write!(f, "You find yourself in {}.\n\nYou have {} HP and {} gold.",
/**/               self.location.borrow().name, self.hp, self.gold)
/**/    }
/**/}
