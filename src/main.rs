mod cell;
mod position;
mod strategy;
mod world;

use crate::cell::Cell;
use crate::position::Position;
use crate::world::World;

fn main() {
    let (rows, columns) = (3, 3);
    let mut world = World::new(rows, columns);

    world.initialize(vec![
        Position::new(1, 0),
        Position::new(1, 1),
        Position::new(1, 2),
    ]);

    print_world(&world);
    print_world(&world.next_gen());
}

fn print_world(world: &World) {
    let cells: Vec<Cell> = (0..world.size.rows)
        .flat_map(|x| {
            println!("");
            return (0..world.size.columns).map(move |y| {
                let position = Position::new(x, y);
                let cell = world.get_cell(position).unwrap().clone();
                if cell.is_alive {
                    print!(" O");
                } else {
                    print!(" -");
                }
                return cell.clone();
            });
        })
        .collect();
    println!("")
}
