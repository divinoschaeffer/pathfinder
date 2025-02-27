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
        for (row,col) in neighbours {
            if !self.cells[row][col].visited {
                non_visited.push((row, col));
            }
        }
        non_visited
    }

    pub fn generate_maze(width: usize, height: usize) -> Maze {
        let mut maze: Maze = Maze::new(width, height);
        maze.cells[0][0].visited = true;
        maze.path.push((0,0));
        maze._generate_maze(0);
        maze
    }

    pub fn _generate_maze(&mut self, current_cell_position: usize) -> &mut Maze {
        let current_cell = self.path[current_cell_position];
        if self.path.len() != 1 && current_cell == (0,0) {
            return self
        }

        let non_visited_neighbours = self.get_non_visited_neighbours(current_cell.0, current_cell.1);
        if non_visited_neighbours.len() == 0 {
            return self._generate_maze(current_cell_position - 1)
        }

        let mut rng = rand::rng();
        let random_number: usize = rng.random_range(0..non_visited_neighbours.len());
        let selected_cell = non_visited_neighbours[random_number];
        self.cells[selected_cell.0][selected_cell.1].visited = true;

        self.path.push(selected_cell);
        self._generate_maze(current_cell_position  + 1)
    }
}
