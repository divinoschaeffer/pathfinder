use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use crate::maze::Maze;
use color_eyre::eyre::WrapErr;
use ratatui::crossterm::event;
use ratatui::crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::widgets::{Block, Borders};
use ratatui::{
    style::{Color, Stylize},
    prelude::*,
    widgets::canvas::{Canvas, Line},
};
use ratatui::{DefaultTerminal, Frame};
use ratatui::crossterm::style::Stylize;
use ratatui::widgets::canvas::Rectangle;
use crate::right_hand::RightHand;

#[derive(Debug)]
pub struct App {
    exit: bool,
    pub maze: Rc<RefCell<Maze>>,
    pub solver : RightHand,
}

impl App {

    pub fn new(maze: Rc<RefCell<Maze>>) -> Self {
        Self {
            exit: false,
            maze: maze.clone(),
            solver: RightHand::new(maze.clone()),
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> color_eyre::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events().wrap_err("Failed to handle events")?;
        }
        Ok(())
    }


    pub fn draw(&self, frame: &mut Frame) {
        let area = frame.area();

        let block = Block::default()
            .title("Maze")
            .borders(Borders::ALL);

        frame.render_widget(block, area);

        draw_maze(self.maze.borrow().deref(), area, frame.buffer_mut());
    }

    fn handle_events(&mut self) -> color_eyre::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => self
                .handle_key_events(key_event)
                .wrap_err_with(|| format!("Failed to handle key events: {:#?}", key_event)),
            _ => Ok(())
        }
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> color_eyre::Result<()> {
        match key.code {
            KeyCode::Char('q') if key.modifiers == KeyModifiers::CONTROL => {
                self.exit = true;
            },
            KeyCode::Char('a') => {
                self.solver.automatic_execution();
            },
            KeyCode::Right => {
                self.solver.step();
            },
            _ => ()
        }
        Ok(())
    }
}

pub fn draw_maze(maze: &Maze, area: Rect, buf: &mut Buffer) {
    let width = maze.width as f64;
    let height = maze.height as f64;

    let instructions = text::Line::from(vec![
        " One Iteration ".into(),
        "<Right>".blue().bold(),
        " Automatic ".into(),
        "<A>".blue().bold(),
        " Quit ".into(),
        " <Ctrl-Q> ".blue().bold(),
    ]);

    let canvas = Canvas::default()
        .block(Block::default().title("Maze").borders(Borders::ALL).title_bottom(instructions.centered()))
        .x_bounds([0.0, width])
        .y_bounds([0.0, height])
        .paint(|ctx| {
            let current_cell = maze.current_cell;
            let exit_cell = (maze.height - 1, maze.width - 1);

            for (i, row) in maze.cells.iter().enumerate() {
                for (j, cell) in row.iter().enumerate() {
                    let x = j as f64;
                    let y = i as f64;

                    if cell.visited {
                        ctx.draw(&Rectangle {
                            x,
                            y,
                            width: 1.0,
                            height: 1.0,
                            color: Color::DarkGray,
                        });
                    }

                    if cell.top_wall {
                        ctx.draw(&Line { x1: x, y1: y, x2: x + 1.0, y2: y, color: Color::White });
                    }
                    if cell.bottom_wall {
                        ctx.draw(&Line { x1: x, y1: y + 1.0, x2: x + 1.0, y2: y + 1.0, color: Color::White });
                    }
                    if cell.left_wall {
                        ctx.draw(&Line { x1: x, y1: y, x2: x, y2: y + 1.0, color: Color::White });
                    }
                    if cell.right_wall {
                        ctx.draw(&Line { x1: x + 1.0, y1: y, x2: x + 1.0, y2: y + 1.0, color: Color::White });
                    }
                }
            }

            ctx.print(0.5, 0.5, "S".green());

            if current_cell == exit_cell {
                ctx.print(
                    current_cell.1 as f64 + 0.5,
                    current_cell.0 as f64 + 0.5,
                    "PE".blue(),
                )
            } else {
                ctx.print(
                    current_cell.1 as f64 + 0.5,
                    current_cell.0 as f64 + 0.5,
                    "P".yellow(),
                );

                ctx.print(
                    exit_cell.1 as f64 + 0.5,
                    exit_cell.0 as f64 + 0.5,
                    "E".red(),
                );
            }
        });

    canvas.render(area, buf);
}

