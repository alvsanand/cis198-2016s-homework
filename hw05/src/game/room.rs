use std::rc::Rc;
use std::cell::RefCell;

use super::curio::Curio;
use super::hall::Hall;

pub struct Room {
    pub name: String,
    pub contents: Vec<Curio>,
    pub halls: Vec<Rc<Hall>>,
    pub wumpus: bool,
}

impl PartialEq for Room {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Room {}

impl Room {
    pub fn new(name: String, contents: Vec<Curio>, halls: Vec<Rc<Hall>>, wumpus: bool) -> Room {
        Room {
            name: name,
            contents: contents,
            halls: halls,
            wumpus: wumpus,
        }
    }

    pub fn neighbors_string(&self) -> String {
        let mut neighbors: Vec<String> = vec![];

        for hall in &self.halls {
            let other_room = hall.other(self);
            neighbors.push(other_room.borrow().name.clone());
        }
        neighbors.join(",").to_string()
    }

    pub fn find_room(&self, room: String) -> Result<Rc<RefCell<Room>>, ()> {
        for hall in &self.halls {
            let other_room = hall.other(self);

            if (*other_room).borrow().name.to_lowercase() == room {
                return Ok(other_room.clone());
            }
        }

        println!("Room {} not found", room);

        Err(())
    }
}
