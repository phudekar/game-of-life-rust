use crate::position::Position;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Cell {
    pub position: Position,
    pub is_alive: bool,
}

impl Cell {
    pub fn new(x: usize, y: usize) -> Cell {
         Cell {
            position: Position::new(x, y),
            is_alive: false,
        }
    }

    pub fn make_alive(&self) -> Cell {
        Cell {
            position: self.position,
            is_alive: true,
        }
    }
}

#[test]
fn cell_should_have_position() {
    let cell = Cell::new(2, 3);
    assert_eq!(2, cell.position.x);
    assert_eq!(3, cell.position.y);
}

#[test]
fn cell_should_be_in_dead_state_initially() {
    let cell = Cell::new(2, 3);
    assert_eq!(false, cell.is_alive);
}
