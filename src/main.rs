use rand::random_bool;

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
    for i in 0..max {
        for j in 0..max {
            let cell = Cell {
                alive: random_bool(0.5),
                position: Point { x: i, y: j},
            };
            cells.push(cell);
        }
    }
    cells
}

fn main() {
    let cells = create_cells(10);
    let mut count: u8 = 0;

    for cell in cells {
        if count % 9 == 0 {
            print!("\n");
        }
        if cell.alive == true { 
            print!("#")
        } else {
            print!("%")
        }
        count += 1;
    }
}
