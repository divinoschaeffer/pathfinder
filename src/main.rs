use crate::maze::Maze;

mod cell;
mod maze;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let maze: Maze = Maze::generate_maze(20,20);
}
