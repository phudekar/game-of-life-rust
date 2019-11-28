#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Size {
    pub rows: usize,
    pub columns: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Position {
        return Position { x, y };
    }

    pub fn get_neighbors(&self, bounds: Size) -> Vec<Position> {
        let one_less_x = self.x.checked_sub(1).unwrap_or(bounds.rows);
        let one_less_y = self.y.checked_sub(1).unwrap_or(bounds.columns);

        vec![
            Position::new(one_less_x, one_less_y),
            Position::new(one_less_x, self.y),
            Position::new(one_less_x, self.y + 1),
            Position::new(self.x, one_less_y),
            Position::new(self.x, self.y + 1),
            Position::new(self.x + 1, one_less_y),
            Position::new(self.x + 1, self.y),
            Position::new(self.x + 1, self.y + 1),
        ]
        .into_iter()
        .filter(|p| p.x >= 0 && p.x < bounds.rows && p.y >= 0 && p.y < bounds.columns)
        .collect()
    }
}

#[test]
fn should_return_neighbors_of_a_cell() {
    let (rows, columns) = (3, 5);
    let bounds = Size { rows, columns };

    let position = Position::new(1, 1);
    let neighbors = position.get_neighbors(bounds);

    assert_eq!(8, neighbors.len());
}

#[test]
fn should_return_neighbors_of_cell_at_corner() {
    let (rows, columns) = (3, 5);
    let bounds = Size { rows, columns };

    let position = Position::new(0, 0);
    let neighbors = position.get_neighbors(bounds);

    assert_eq!(3, neighbors.len());

    let position = Position::new(2, 4);
    let neighbors = position.get_neighbors(bounds);

    assert_eq!(3, neighbors.len());
}
