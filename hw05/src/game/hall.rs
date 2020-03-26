use std::cell::RefCell;
use std::rc::Rc;

use super::room::Room;

pub struct Hall {
    pub left: Rc<RefCell<Room>>,
    pub right: Rc<RefCell<Room>>,
}

impl Hall {
    pub fn new(left: Rc<RefCell<Room>>, right: Rc<RefCell<Room>>) -> Hall {
        Hall {
            left: left,
            right: right,
        }
    }

    /// Given a Room `room`, find the room at the other end of Hall `self`.
    pub fn other(&self, room: &Room) -> Rc<RefCell<Room>> {
        if self.left.borrow().eq(room) {
            self.right.clone()
        } else {
            self.left.clone()
        }
    }
}
