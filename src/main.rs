use rand::{random, random_bool};

struct Point {
    x: u8,
    y: u8,
}

struct Cell {
    alive: bool,
    position: Point,
}

fn create_cells(max: u8) -> Vec<Cell> {
    let mut cells: Vec<Cell> = Vec::new(); 
    for _i in 0..max {
        let cell = Cell {
            alive: random_bool(0.5),
            position: Point { x: random::<u8>(), y: random::<u8>()},
        };
        cells.push(cell);
    }
    cells
}

fn main() {
    let cells = create_cells(10);

}
