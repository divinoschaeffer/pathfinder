use rand::Rng;
use crate::cell::Cell;

#[derive(Debug)]
pub struct Maze {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Vec<Cell>>,
    pub path: Vec<(usize, usize)>,
}

impl Maze {
    pub fn new(width: usize, height: usize) -> Maze {
        let cells = (0..height)
            .map(|_| (0..width).map(|_| Cell::default()).collect())
            .collect();

        Maze {
            width,
            height,
            cells,
            path: Vec::new(),
        }
    }

    pub fn get_neighbours(&self, i: usize, j: usize) -> Vec<(usize, usize)> {
        let mut neighbours = Vec::new();

        if i > 0 {
            neighbours.push((i - 1, j));
        }
        if i + 1 < self.height {
            neighbours.push((i + 1, j));
        }
        if j > 0 {
            neighbours.push((i, j - 1));
        }
        if j + 1 < self.width {
            neighbours.push((i, j + 1));
        }

        neighbours
    }

    pub fn get_non_visited_neighbours(&self, i: usize, j: usize) -> Vec<(usize, usize)> {
        let mut non_visited = Vec::new();
        let neighbours = self.get_neighbours(i, j);
        for (row, col) in neighbours {
            if !self.cells[row][col].visited {
                non_visited.push((row, col));
            }
        }
        non_visited
    }

    pub fn generate_maze(width: usize, height: usize) -> Maze {
        let mut maze = Maze::new(width, height);

        maze.cells[0][0].visited = true;

        let mut stack = Vec::new();
        stack.push((0, 0));
        maze.path.push((0, 0));

        while let Some(current_cell) = stack.last().copied() {
            let non_visited_neighbours = maze.get_non_visited_neighbours(current_cell.0, current_cell.1);

            if non_visited_neighbours.is_empty() {
                stack.pop();

                if stack.is_empty() {
                    break;
                }
            } else {
                let mut rng = rand::rng();
                let random_number: usize = rng.random_range(0..non_visited_neighbours.len());
                let selected_cell = non_visited_neighbours[random_number];

                maze.cells[selected_cell.0][selected_cell.1].visited = true;

                maze.open_adjacent_wall(current_cell, selected_cell);

                stack.push(selected_cell);
                maze.path.push(selected_cell);
            }
        }

        maze
    }

    pub fn open_adjacent_wall(
        &mut self,
        first: (usize, usize),
        second: (usize, usize),
    ) {
        if first == second {
            return;
        }

        let ((row1, col1), (row2, col2)) = (first, second);

        if row1.abs_diff(row2) + col1.abs_diff(col2) != 1 {
            return;
        }

        if row1 == row2 {
            let (left, right) = if col1 < col2 { (first, second) } else { (second, first) };
            self.cells[left.0][left.1].right_wall = false;
            self.cells[right.0][right.1].left_wall = false;
        } else {
            let (top, bottom) = if row1 < row2 { (first, second) } else { (second, first) };
            self.cells[top.0][top.1].bottom_wall = false;
            self.cells[bottom.0][bottom.1].top_wall = false;
        }
    }
}
