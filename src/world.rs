use crate::cell::Cell;
use crate::position::Position;

pub struct World {
    size: Size,
    cells: Vec<Cell>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Size {
    rows: usize,
    columns: usize,
}

impl World {
    fn new(rows: usize, columns: usize) -> World {
        let cells = (0..rows)
            .flat_map(move |x| (0..columns).map(move |y| return Cell::new(x, y)))
            .collect();

        World {
            size: Size {
                rows: rows,
                columns: columns,
            },
            cells: cells,
        }
    }

    fn initialize(&mut self, aliveCellPositions: Vec<Position>) {
        self.cells = self
            .cells
            .iter()
            .map(|cell| {
                let new_cell = Cell::new(cell.position.x, cell.position.y);
                if aliveCellPositions.contains(&cell.position) {
                    return new_cell.make_alive();
                } else {
                    return new_cell;
                }
            })
            .collect();
    }

    fn get_cell(&self, position: Position) -> Option<&Cell> {
        self.cells.iter().find(|cell| cell.position == position)
    }

    fn get_neighbors(&self, position: Position) -> Vec<&Cell> {
        let one_less_x = position.x.checked_sub(1).unwrap_or(self.size.rows);
        let one_less_y = position.y.checked_sub(1).unwrap_or(self.size.columns);

        let positions: Vec<Position> = vec![
            Position::new(one_less_x, one_less_y),
            Position::new(one_less_x, position.y),
            Position::new(one_less_x, position.y + 1),
            Position::new(position.x, one_less_y),
            Position::new(position.x, position.y + 1),
            Position::new(position.x + 1, one_less_y),
            Position::new(position.x + 1, position.y),
            Position::new(position.x + 1, position.y + 1),
        ]
        .into_iter()
        .filter(|p| p.x >= 0 && p.y >= 0)
        .collect();

        self.cells
            .iter()
            .filter(|cell| positions.contains(&cell.position))
            .collect()
    }
}

#[test]
fn should_create_world_with_given_dimension() {
    let (rows, columns) = (3, 5);
    let world = World::new(rows, columns);
    assert_eq!(Size { rows, columns }, world.size)
}

#[test]
fn should_create_dead_cells_by_default() {
    let (rows, columns) = (3, 5);
    let world = World::new(rows, columns);
    assert_eq!(false, world.get_cell(Position::new(2, 3)).unwrap().is_alive)
}

#[test]
fn should_initialize_with_alive_cells() {
    let (rows, columns) = (3, 5);
    let mut world = World::new(rows, columns);
    let position = Position::new(2, 3);

    world.initialize(vec![position]);

    assert_eq!(
        Some(&Cell {
            position,
            is_alive: true
        }),
        world.get_cell(position)
    )
}

#[test]
fn should_return_neighbors_of_a_cell() {
    let (rows, columns) = (3, 5);
    let world = World::new(rows, columns);

    let position = Position::new(1, 1);
    let neighbors = world.get_neighbors(position);

    assert_eq!(8, neighbors.len());
}

#[test]
fn should_return_neighbors_of_cell_at_corner() {
    let (rows, columns) = (3, 5);
    let world = World::new(rows, columns);

    let position = Position::new(0, 0);
    let neighbors = world.get_neighbors(position);

    assert_eq!(3, neighbors.len());

    let position = Position::new(2, 4);
    let neighbors = world.get_neighbors(position);

    assert_eq!(3, neighbors.len());
}
