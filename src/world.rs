use crate::cell::Cell;
use crate::position::Position;
use crate::position::Size;
use crate::strategy::Strategy;

pub struct World {
    size: Size,
    cells: Vec<Cell>,
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

    fn next_gen(&self) -> World {
        let cells = self
            .cells
            .iter()
            .map(|cell| {
                let neighbours = cell
                    .position
                    .neighbors(self.size)
                    .iter()
                    .map(|position| self.get_cell(*position))
                    .filter(|c| (*c).is_some())
                    .map(|c: Option<&Cell>| c.unwrap().clone())
                    .collect();

                cell.next_state(neighbours)
            })
            .collect();

        World {
            size: self.size,
            cells,
        }
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
fn should_return_next_generation_for_single_cell() {
    let (rows, columns) = (3, 5);
    let mut world = World::new(rows, columns);
    let position = Position::new(2, 3);

    world.initialize(vec![position]);

    let next_gen = world.next_gen();

    assert_eq!(false, next_gen.get_cell(position).unwrap().is_alive)
}

#[test]
fn should_return_next_generation_with_more_cells() {
    let (rows, columns) = (3, 5);
    let mut world = World::new(rows, columns);

    world.initialize(vec![
        Position::new(0, 1),
        Position::new(1, 1),
        Position::new(2, 1),
    ]);

    let next_gen = world.next_gen();

    assert_eq!(
        false,
        next_gen.get_cell(Position::new(0, 1),).unwrap().is_alive
    );
    assert_eq!(
        true,
        next_gen.get_cell(Position::new(1, 1),).unwrap().is_alive
    );
    assert_eq!(
        false,
        next_gen.get_cell(Position::new(2, 1),).unwrap().is_alive
    );
    assert_eq!(
        true,
        next_gen.get_cell(Position::new(1, 0),).unwrap().is_alive
    );
    assert_eq!(
        true,
        next_gen.get_cell(Position::new(1, 2),).unwrap().is_alive
    );
}
