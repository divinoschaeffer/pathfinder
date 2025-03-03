use rand::Rng;
use crate::cell::Cell;

#[derive(Debug)]
pub struct Maze {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Vec<Cell>>,
    pub path: Vec<(usize, usize)>,
    pub current_cell: (usize, usize),
    pub exit: (usize, usize),
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
            current_cell: (0,0),
            exit: (width - 1, height - 1),
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

    pub fn reset_visited_cells(&mut self) {
        for row in &mut self.cells {
            for cell in row {
                cell.visited = false;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_maze() {
        let width = 10;
        let height = 10;
        let maze = Maze::generate_maze(width, height);

        assert_eq!(maze.width, width);
        assert_eq!(maze.height, height);
        assert_eq!(maze.cells.len(), height);
        assert_eq!(maze.cells[0].len(), width);

        for row in &maze.cells {
            for cell in row {
                assert!(cell.visited, "All cells should be visited in a perfect maze.");
            }
        }

        assert_eq!(maze.path.len(), width * height, "The path should cover the entire maze.");
    }

    #[test]
    fn test_get_neighbours() {
        let maze = Maze::new(5, 5);

        // Cell in the center (should have 4 neighbors)
        let neighbours = maze.get_neighbours(2, 2);
        let expected = vec![(1, 2), (3, 2), (2, 1), (2, 3)];
        assert_eq!(neighbours, expected, "The center cell should have 4 neighbors");

        // Top-left corner (should have 2 neighbors)
        let neighbours = maze.get_neighbours(0, 0);
        let expected = vec![(1, 0), (0, 1)];
        assert_eq!(neighbours, expected, "The top-left cell should have 2 neighbors");

        // Bottom-right corner (should have 2 neighbors)
        let neighbours = maze.get_neighbours(4, 4);
        let expected = vec![(3, 4), (4, 3)];
        assert_eq!(neighbours, expected, "The bottom-right cell should have 2 neighbors");

        // Left edge (should have 3 neighbors)
        let neighbours = maze.get_neighbours(2, 0);
        let expected = vec![(1, 0), (3, 0), (2, 1)];
        assert_eq!(neighbours, expected, "A left-edge cell should have 3 neighbors");

        // Bottom edge (should have 3 neighbors)
        let neighbours = maze.get_neighbours(4, 2);
        let expected = vec![(3, 2), (4, 1), (4, 3)];
        assert_eq!(neighbours, expected, "A bottom-edge cell should have 3 neighbors");
    }

    #[test]
    fn test_get_non_visited_neighbours() {
        let mut maze = Maze::new(5, 5);

        // Initially, all cells are unvisited, so all neighbors should be returned.
        let non_visited = maze.get_non_visited_neighbours(2, 2);
        let expected = vec![(1, 2), (3, 2), (2, 1), (2, 3)];
        assert_eq!(non_visited, expected, "All neighbors should be unvisited at the start");

        // Mark (1,2) as visited
        maze.cells[1][2].visited = true;
        let non_visited = maze.get_non_visited_neighbours(2, 2);
        let expected = vec![(3, 2), (2, 1), (2, 3)];
        assert_eq!(non_visited, expected, "Cell (1,2) should no longer be in the list");

        // Mark all neighbors as visited
        maze.cells[3][2].visited = true;
        maze.cells[2][1].visited = true;
        maze.cells[2][3].visited = true;
        let non_visited = maze.get_non_visited_neighbours(2, 2);
        assert!(non_visited.is_empty(), "There should be no unvisited neighbors");

        // Test on a corner (0,0), initially all neighbors should be unvisited
        let non_visited = maze.get_non_visited_neighbours(0, 0);
        let expected = vec![(1, 0), (0, 1)];
        assert_eq!(non_visited, expected, "The top-left corner should have 2 unvisited neighbors");

        // Mark (1,0) as visited
        maze.cells[1][0].visited = true;
        let non_visited = maze.get_non_visited_neighbours(0, 0);
        let expected = vec![(0, 1)];
        assert_eq!(non_visited, expected, "Only (0,1) should be unvisited now");

        // Mark (0,1) as visited
        maze.cells[0][1].visited = true;
        let non_visited = maze.get_non_visited_neighbours(0, 0);
        assert!(non_visited.is_empty(), "No unvisited neighbors should be left for (0,0)");
    }

    #[test]
    fn test_open_adjacent_wall_horizontal() {
        let mut maze = Maze::new(5, 5);

        // Open wall between (2,2) and (2,3)
        maze.open_adjacent_wall((2, 2), (2, 3));

        // Check walls
        assert!(!maze.cells[2][2].right_wall, "Right wall of (2,2) should be open");
        assert!(!maze.cells[2][3].left_wall, "Left wall of (2,3) should be open");
    }

    #[test]
    fn test_open_adjacent_wall_vertical() {
        let mut maze = Maze::new(5, 5);

        // Open wall between (2,2) and (3,2)
        maze.open_adjacent_wall((2, 2), (3, 2));

        // Check walls
        assert!(!maze.cells[2][2].bottom_wall, "Bottom wall of (2,2) should be open");
        assert!(!maze.cells[3][2].top_wall, "Top wall of (3,2) should be open");
    }

    #[test]
    fn test_open_adjacent_wall_non_adjacent() {
        let mut maze = Maze::new(5, 5);

        // Attempt to open wall between non-adjacent cells (should not change anything)
        maze.open_adjacent_wall((2, 2), (4, 2));

        // Check that walls remain closed
        assert!(maze.cells[2][2].bottom_wall, "Bottom wall of (2,2) should remain closed");
        assert!(maze.cells[4][2].top_wall, "Top wall of (4,2) should remain closed");
    }

    #[test]
    fn test_open_adjacent_wall_same_cell() {
        let mut maze = Maze::new(5, 5);

        // Opening wall between the same cell should do nothing
        maze.open_adjacent_wall((2, 2), (2, 2));

        // Ensure all walls remain closed
        assert!(maze.cells[2][2].top_wall, "Top wall of (2,2) should remain closed");
        assert!(maze.cells[2][2].bottom_wall, "Bottom wall of (2,2) should remain closed");
        assert!(maze.cells[2][2].left_wall, "Left wall of (2,2) should remain closed");
        assert!(maze.cells[2][2].right_wall, "Right wall of (2,2) should remain closed");
    }
}
