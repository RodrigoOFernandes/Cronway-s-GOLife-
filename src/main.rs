use rand::random_bool;

struct Cell {
    alive: bool,
}

fn create_cells(max: u8) -> Vec<Cell> {
    let mut cells: Vec<Cell> = Vec::new(); 
    for _i in 0..max {
        for _j in 0..max {
            let cell = Cell {
                alive: random_bool(0.5),
            };
            cells.push(cell);
        }
    }
    cells
}

fn get_neighbors(index: usize, max: u8) -> Vec<usize> {
    let i = index / max as usize;
    let j = index % max as usize;

    let mut neighbors = Vec::new();

    for di in -1..=1 {
        for dj in -1..=1 {
            if di == 0 && dj == 0 {
                continue;
            }
            
            let ni = i as isize + di;
            let nj = j as isize + dj;

            if ni >= 0 && ni < max as isize && nj >= 0 && nj < max as isize {
                let neighbor_index = ni as usize * max as usize +nj as usize;

                neighbors.push(neighbor_index);
            }
        }
    }
    
    neighbors
}

fn survivors(mut cells: Vec<Cell>, max: u8) -> Vec<Cell> {
    for i in 0..cells.len(){
        let neighbors = get_neighbors(i, max);

        let count = neighbors.iter().filter(|&&idx| cells[idx].alive == true).count();
    
        if (count == 3) || (count == 2 && cells[i].alive == true)  {
            cells[i].alive = true;
        } else {
            cells[i].alive = false;
        }
    }
    cells
}

fn main() {
    let max: u8 = 10;
    let cells = create_cells(max);
    let mut count: u8 = 0;

    for cell in cells {
        if count % max == 0 {
            print!("\n");
        }
        if cell.alive == true { 
            print!("#")
        } else {
            print!("%")
        }
        count += 1;
    }
    print!("\n");
}
