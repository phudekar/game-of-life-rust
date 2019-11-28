use crate::cell::Cell;
use crate::position::Position;

trait Strategy {
    fn next_state(&self, neighbors: Vec<Cell>) -> Cell;
}

impl Strategy for Cell {
    fn next_state(&self, neighbors: Vec<Cell>) -> Cell {
        let alive_neighbors = neighbors.iter().filter(|&n| n.is_alive).count();
        match alive_neighbors {
            2 | 3 if self.is_alive => {
                return Cell {
                    position: self.position,
                    is_alive: true,
                }
            }
            3 if !self.is_alive => {
                return Cell {
                    position: self.position,
                    is_alive: true,
                }
            }
            _ => return Cell::new(self.position.x, self.position.y),
        }
    }
}

#[test]
fn dead_cell_should_stay_dead_without_any_living_neighbor() {
    let cell = Cell::new(3, 4);
    let next = cell.next_state(vec![]);
    assert_eq!(false, next.is_alive);
}

#[test]
fn alive_cell_should_become_dead_with_less_than_2_neighbors() {
    let cell = alive_cell();
    let next = cell.next_state(vec![alive_cell()]);
    assert_eq!(false, next.is_alive);
}

#[test]
fn alive_cell_should_become_dead_with_more_than_3_neighbors() {
    let cell = alive_cell();
    let next = cell.next_state(vec![
        alive_cell(),
        alive_cell(),
        alive_cell(),
        alive_cell(),
        dead_cell(),
    ]);
    assert_eq!(false, next.is_alive);
}

#[test]
fn alive_cell_should_stay_alive_with_3_neighbors() {
    let cell = alive_cell();
    let next = cell.next_state(vec![alive_cell(), alive_cell(), alive_cell(), dead_cell()]);
    assert_eq!(true, next.is_alive);
}

#[test]
fn alive_cell_should_stay_alive_with_2_neighbors() {
    let cell = alive_cell();
    let next = cell.next_state(vec![alive_cell(), alive_cell(), dead_cell()]);
    assert_eq!(true, next.is_alive);
}

#[test]
fn dead_cell_should_become_alive_with_exact_3_neighbors() {
    let cell = dead_cell();
    let next = cell.next_state(vec![alive_cell(), alive_cell(), alive_cell(), dead_cell()]);
    assert_eq!(true, next.is_alive);
}

#[test]
fn dead_cell_should_stay_dead_with_2_neighbors() {
    let cell = dead_cell();
    let next = cell.next_state(vec![alive_cell(), alive_cell(), dead_cell()]);
    assert_eq!(false, next.is_alive);
}

fn alive_cell() -> Cell {
    return Cell {
        position: Position::new(0, 0),
        is_alive: true,
    };
}

fn dead_cell() -> Cell {
    return Cell {
        position: Position::new(0, 0),
        is_alive: false,
    };
}
