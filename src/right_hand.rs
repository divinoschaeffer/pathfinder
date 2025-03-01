use std::cell::RefCell;
use std::rc::Rc;
use crate::maze::Maze;

#[derive(Debug)]
pub struct RightHand {
    pub maze: Rc<RefCell<Maze>>,
    pub current_direction: Direction,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl RightHand {
    pub fn new(maze: Rc<RefCell<Maze>>) -> Self {
        RightHand { maze, current_direction: Direction::North }
    }

    pub fn step(&mut self) {
        let mut maze = self.maze.borrow_mut();
        let (next_cell, next_direction) = get_next_cell(&maze, self.current_direction);
        maze.current_cell = next_cell;
        self.current_direction = next_direction;
    }

    pub fn automatic_execution(&mut self) {
        while self.maze.borrow().current_cell != self.maze.borrow().exit {
            self.step();
        }
    }
}

fn get_next_cell(maze: &Maze, current_direction: Direction) -> ((usize, usize), Direction) {
    let (row, column) = maze.current_cell;
    let rows = maze.cells.len();
    let cols = if rows > 0 { maze.cells[0].len() } else { 0 };

    match current_direction {
        Direction::North => {
            if !maze.cells[row][column].right_wall && column + 1 < cols {
                return ((row, column + 1), Direction::East);
            }
            else if !maze.cells[row][column].top_wall && row > 0 {
                return ((row - 1, column), Direction::North);
            }
            else if !maze.cells[row][column].left_wall && column > 0 {
                return ((row, column - 1), Direction::West);
            }
            else if !maze.cells[row][column].bottom_wall && row + 1 < rows {
                return ((row + 1, column), Direction::South);
            }
        },
        Direction::East => {
            if !maze.cells[row][column].bottom_wall && row + 1 < rows {
                return ((row + 1, column), Direction::South);
            }
            else if !maze.cells[row][column].right_wall && column + 1 < cols {
                return ((row, column + 1), Direction::East);
            }
            else if !maze.cells[row][column].top_wall && row > 0 {
                return ((row - 1, column), Direction::North);
            }
            else if !maze.cells[row][column].left_wall && column > 0 {
                return ((row, column - 1), Direction::West);
            }
        },
        Direction::South => {
            if !maze.cells[row][column].left_wall && column > 0 {
                return ((row, column - 1), Direction::West);
            }
            else if !maze.cells[row][column].bottom_wall && row + 1 < rows {
                return ((row + 1, column), Direction::South);
            }
            else if !maze.cells[row][column].right_wall && column + 1 < cols {
                return ((row, column + 1), Direction::East);
            }
            else if !maze.cells[row][column].top_wall && row > 0 {
                return ((row - 1, column), Direction::North);
            }
        },
        Direction::West => {
            if !maze.cells[row][column].top_wall && row > 0 {
                return ((row - 1, column), Direction::North);
            }
            else if !maze.cells[row][column].left_wall && column > 0 {
                return ((row, column - 1), Direction::West);
            }
            else if !maze.cells[row][column].bottom_wall && row + 1 < rows {
                return ((row + 1, column), Direction::South);
            }
            else if !maze.cells[row][column].right_wall && column + 1 < cols {
                return ((row, column + 1), Direction::East);
            }
        },
    }

    ((row, column), current_direction)
}
