use crate::entity::Entity;
use crate::types::{NumSquares};
use crate::square::Square;


pub struct Position {
    boxes: [Option<Entity>; NumSquares],
    Square man_pos
};