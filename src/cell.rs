#[derive(Debug)]
pub struct Cell {
    pub cell_type: CellType
}

#[derive(Debug)]
pub enum CellType {
    Path,
    Start,
    Finish,
    Inaccessible,
}
