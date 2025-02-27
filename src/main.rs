use crate::maze::Maze;
use clap::{arg, command, value_parser};

mod cell;
mod maze;

fn main() {
    let matches = command!()
        .author("Schaeffer Divino, divinoschaeffer@gmail.com")
        .arg(arg!(-d --dimensions <DIM> "Dimensions of the maze")
            .required(true)
            .value_parser(value_parser!(usize))
            .num_args(2))
        .get_matches();

    let (width, height) = if let Some(mut dims) = matches.get_many::<usize>("dimensions") {
        (*dims.next().unwrap(), *dims.next().unwrap())
    } else {
        (20, 20)
    };

    let maze: Maze = Maze::generate_maze(width,height);
}
