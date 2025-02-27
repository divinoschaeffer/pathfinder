use crate::maze::Maze;
use clap::{arg, command, value_parser};
use color_eyre::Result;
use ratatui::{DefaultTerminal, Frame};
use ratatui::crossterm::event;
use ratatui::crossterm::event::Event;

mod cell;
mod maze;

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

    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    let maze: Maze = Maze::generate_maze(width,height);

    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(render)?;
        if matches!(event::read()?, Event::Key(_)) {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    frame.render_widget("hello world", frame.area());
}
