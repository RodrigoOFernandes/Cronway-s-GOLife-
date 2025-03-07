use rand::random_bool;
use clearscreen::clear;
use std::io::stdin;

#[derive(Clone, Copy)]
struct Cell {
    alive: bool,
}

fn create_cells(max: u8, probability: f64) -> Vec<Cell> {
    let mut cells: Vec<Cell> = Vec::new();
    for _i in 0..max {
        for _j in 0..max {
            let cell = Cell {
                alive: random_bool(probability),
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
                let neighbor_index = ni as usize * max as usize + nj as usize;
                neighbors.push(neighbor_index);
            }
        }
    }

    neighbors
}

fn survivors(cells: Vec<Cell>, max: u8) -> Vec<Cell> {
    let mut new_cells = cells.clone(); 

    for i in 0..cells.len() {
        let neighbors = get_neighbors(i, max);

        let count = neighbors.iter().filter(|&idx| cells[*idx].alive).count();

        if count == 3 || (count == 2 && cells[i].alive) {
            new_cells[i].alive = true;
        } else {
            new_cells[i].alive = false;
        }
    }

    new_cells
}

fn main() {
    println!("WELCOME TO THE GAME OF LIFE\n");
    println!("How many percentage of alive cells do you want?(Percentage from 0.0 -> 1.0)");
    let mut buffer = String::new();
    let _ = stdin().read_line(&mut buffer);

    let max: u8 = 30;
    let probability = buffer.trim().parse::<f64>().unwrap();
    let mut cells = create_cells(max, probability);

    loop {
        for (count, cell) in cells.iter().enumerate() {
            if count % max as usize == 0 {
                println!();
            }
            if cell.alive {
                print!("\x1b[38;2;0;255;0m■\x1b[0m");
            } else {
                print!("□");
            }
        }
        println!();

        cells = survivors(cells, max);
        
        std::thread::sleep(std::time::Duration::from_millis(500));
        
        clear().expect("failed to clear screen");
    }
}
