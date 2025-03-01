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

#[derive(Debug)]
pub struct App {
    exit: bool,
    pub maze: Maze,
}

impl App {

    pub fn new(maze: Maze) -> Self {
        Self {
            exit: false,
            maze
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

        draw_maze(&self.maze, area, frame.buffer_mut());
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
            _ => ()
        }
        Ok(())
    }
}

pub fn draw_maze(maze: &Maze, area: Rect, buf: &mut Buffer) {
    let width = maze.width as f64;
    let height = maze.height as f64;

    let canvas = Canvas::default()
        .block(Block::default().title("Maze").borders(Borders::ALL))
        .x_bounds([0.0, width])
        .y_bounds([0.0, height])
        .paint(|ctx| {
            for (i, row) in maze.cells.iter().enumerate() {
                for (j, cell) in row.iter().enumerate() {
                    let x = j as f64;
                    let y = i as f64;

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
        });

    canvas.render(area, buf);
}
