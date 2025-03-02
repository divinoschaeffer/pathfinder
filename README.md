# Maze Pathfinding Visualizer

This project is designed to demonstrate the execution of a pathfinding algorithm on a maze. The goal is to visually showcase how different algorithms navigate through a labyrinth, solving it step by step.

## Features
- Generates a maze based on user-specified dimensions.
- Implements a pathfinding algorithm to find a solution.
- Displays the execution process in real-time.
- Future updates will include multiple pathfinding algorithms for comparison.

## Upcoming Features
- Support for additional algorithms such as A*, Dijkstra, and BFS.
- Performance comparison between different algorithms.
- Customizable maze generation options.

## Usage
To run the program, specify the dimensions of the maze as input parameters:
```
cargo run -- <width> <height>
```
Example:
```
cargo run -- 20 20
```
This will generate a 20x20 maze and execute the pathfinding algorithm.

## Dependencies
- Rust
- Ratatui (for visualization)
- Rand (for maze generation)

## Contributing
Contributions are welcome! Feel free to submit issues or pull requests to enhance the project.

## License
This project is open-source and available under the MIT License.

