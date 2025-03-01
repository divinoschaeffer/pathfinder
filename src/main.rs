use std::cell::RefCell;
use std::rc::Rc;
use crate::app::App;
use crate::maze::Maze;
use clap::{arg, command, value_parser};
use color_eyre::eyre::eyre;
use color_eyre::Result;

mod cell;
mod maze;
mod app;
mod right_hand;

fn main() -> Result<()>{
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

    if width == 0 || height == 0  || width > 1000 || height > 1000 {
        return Err(eyre!("Invalid dimensions"));
    }

    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let mut maze: Maze = Maze::generate_maze(width,height);
    maze.reset_visited_cells();
    let result = App::new(Rc::new(RefCell::new(maze))).run(&mut terminal);

    ratatui::restore();
    result
}
