mod cell;
mod position;
mod strategy;
mod world;

fn main() {
    greet("world!");
}

fn greet(msg: &str) {
    println!("Hello, {}", msg);
}
