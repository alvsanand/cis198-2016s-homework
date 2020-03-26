use std::cell::RefCell;
use std::io::Read;
use std::rc::Rc;
use std::result;

use rustc_serialize::json::Json;

use super::curio::Curio;
use super::hall::Hall;
use super::room::Room;

pub type Result<T> = result::Result<T, String>;

pub struct Board {
    pub rooms: Vec<Rc<RefCell<Room>>>,
}

impl Board {
    pub fn build_board(reader: &mut dyn Read) -> Result<Board> {
        let mut board = Board { rooms: Vec::new() };

        let board_json: Json =
            Json::from_reader(reader).map_err(|_| "Unable to create JSON reader".to_string())?;

        board
            .parse_rooms(&board_json)
            .map_err(|_| "Unable to parse rooms".to_string())?;
        board
            .parse_halls(&board_json)
            .map_err(|_| "Unable to parse halls".to_string())?;

        Ok(board)
    }

    fn parse_rooms(&mut self, json: &Json) -> Result<()> {
        // Find room list
        let rooms_opt: Option<&Json> = json.find("rooms");
        let json_rooms: &Json = rooms_opt.ok_or("Unable to parse rooms".to_string())?;

        // Parse each room
        let rooms: &Vec<Json> = json_rooms
            .as_array()
            .ok_or("Unable to parse name".to_string())?;
        for r in rooms {
            // Parse room name
            let name_opt: Option<&Json> = r.find("name");
            let json_name: &Json = (name_opt.ok_or("Unable to parse name".to_string()))?;
            let name: &str = json_name
                .as_string()
                .ok_or("Unable to parse name".to_string())?;

            // Parse curios
            let curios_opt: Option<&Json> = r.find("curios");
            let json_curios: &Json = curios_opt.ok_or("Unable to parse curio".to_string())?;
            let n: u64 = json_curios
                .as_u64()
                .ok_or("Unable to parse curio".to_string())?;
            let curios: Vec<Curio> = Curio::generate_n(n as usize);

            // Wumpus?
            let wumpus: bool = {
                if let Some(json_wumpus) = r.find("wumpus") {
                    json_wumpus
                        .as_boolean()
                        .ok_or("Unable to parse Wumpus".to_string())?
                } else {
                    false
                }
            };

            let new_room = Room::new(name.into(), curios, Vec::new(), wumpus);

            // Add the new room to self.rooms
            self.rooms.push(Rc::new(RefCell::new(new_room)));
        }

        Ok(())
    }

    fn parse_halls(&mut self, json: &Json) -> Result<()> {
        // Find hall list
        let halls_opt: Option<&Json> = json.find("halls");
        let json_halls: &Json = halls_opt.ok_or("Unable to parse halls".to_string())?;

        // Parse each hall
        let halls: &Vec<Json> = json_halls
            .as_array()
            .ok_or("Unable to parse halls".to_string())?;
        for h in halls {
            let h: &Vec<Json> = h.as_array().ok_or("Unable to parse halls".to_string())?;
            if h.len() > 2 {
                return Err("Invalid number of rooms per hall".to_string());
            }

            let left = h
                .get(0)
                .map(|v| v.as_u64().map(|v| self.rooms.get(v as usize)))
                .flatten()
                .flatten();
            let right = h
                .get(1)
                .map(|v| v.as_u64().map(|v| self.rooms.get(v as usize)))
                .flatten()
                .flatten();

            if left.is_none() || right.is_none() {
                return Err("Invalid rooms ids".to_string());
            }

            let hall = Hall::new(left.unwrap().clone(), right.unwrap().clone());

            let rc_hall = Rc::new(hall);

            left.unwrap().borrow_mut().halls.push(rc_hall.clone());
            right.unwrap().borrow_mut().halls.push(rc_hall.clone());
        }
        Ok(())
    }

    pub fn spawn_location(&self) -> Rc<RefCell<Room>> {
        self.rooms[0].clone()
    }
}
