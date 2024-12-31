use rand::Rng; // Untuk menghasilkan angka acak.
use std::collections::BinaryHeap;
use std::cmp::Ordering;

// Struktur data untuk merepresentasikan node di grid.
#[derive(Copy, Clone)]
struct Node {
    position: (usize, usize),
    priority: f64, // Prioritas (cost + heuristic).
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl Eq for Node {} // Implementasi kosong untuk memenuhi syarat `BinaryHeap`.

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.priority.partial_cmp(&self.priority).unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Fungsi heuristic untuk estimasi jarak tujuan (menggunakan Manhattan distance).
fn heuristic(a: (usize, usize), b: (usize, usize)) -> f64 {
    ((a.0 as isize - b.0 as isize).abs() + (a.1 as isize - b.1 as isize).abs()) as f64
}

// Fungsi untuk menentukan jalur terbaik dengan probabilitas ketidakpastian.
fn probabilistic_pathfinding(
    grid: &Vec<Vec<i32>>, 
    start: (usize, usize), 
    goal: (usize, usize), 
    sensor_noise: f64,
) -> Option<Vec<(usize, usize)>> {
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut open_set = BinaryHeap::new();
    let mut came_from = vec![vec![None; grid[0].len()]; grid.len()];
    let mut cost_so_far = vec![vec![f64::INFINITY; grid[0].len()]; grid.len()];

    open_set.push(Node {
        position: start,
        priority: heuristic(start, goal),
    });
    cost_so_far[start.0][start.1] = 0.0;

    while let Some(current) = open_set.pop() {
        if current.position == goal {
            let mut path = vec![];
            let mut current_pos = Some(goal);
            while let Some(pos) = current_pos {
                path.push(pos);
                current_pos = came_from[pos.0][pos.1];
            }
            path.reverse();
            return Some(path);
        }

        for dir in &directions {
            let new_x = current.position.0 as isize + dir.0;
            let new_y = current.position.1 as isize + dir.1;

            if new_x >= 0
                && new_x < grid.len() as isize
                && new_y >= 0
                && new_y < grid[0].len() as isize
            {
                let new_position = (new_x as usize, new_y as usize);

                if grid[new_position.0][new_position.1] == 1 {
                    continue; // Abaikan rintangan.
                }

                // Tambahkan ketidakpastian sensor ke biaya.
                let noise: f64 = rand::thread_rng().gen_range(0.0..sensor_noise);
                let new_cost = cost_so_far[current.position.0][current.position.1] + 1.0 + noise;

                if new_cost < cost_so_far[new_position.0][new_position.1] {
                    cost_so_far[new_position.0][new_position.1] = new_cost;
                    came_from[new_position.0][new_position.1] = Some(current.position);
                    open_set.push(Node {
                        position: new_position,
                        priority: new_cost + heuristic(new_position, goal),
                    });
                }
            }
        }
    }

    None // Tidak ada jalur yang ditemukan.
}

// Fungsi untuk mencetak grid dengan jalur yang ditemukan.
fn display_grid(grid: &Vec<Vec<i32>>, path: &Vec<(usize, usize)>) {
    let mut grid_copy = grid.clone();
    for &(x, y) in path {
        grid_copy[x][y] = 8; // Gunakan angka 8 untuk menunjukkan jalur.
    }
    for row in grid_copy {
        for cell in row {
            print!("{} ", cell);
        }
        println!();
    }
}

fn main() {
    // Definisikan grid dengan 0 sebagai jalan bebas dan 1 sebagai rintangan.
    let grid = vec![
        vec![0, 0, 0, 0, 0],
        vec![0, 1, 1, 1, 0],
        vec![0, 0, 0, 1, 0],
        vec![0, 1, 0, 0, 0],
        vec![0, 0, 0, 0, 0],
    ];

    let start = (0, 0);
    let goal = (4, 4);
    let sensor_noise = 0.5; // Ketidakpastian data sensor.

    match probabilistic_pathfinding(&grid, start, goal, sensor_noise) {
        Some(path) => {
            println!("Jalur ditemukan:");
            for step in &path {
                println!("({}, {})", step.0, step.1);
            }
            println!("\nGrid dengan jalur:");
            display_grid(&grid, &path);
        }
        None => println!("Tidak ada jalur yang ditemukan."),
    }
}
