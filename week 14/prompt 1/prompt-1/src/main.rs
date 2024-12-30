use std::collections::BinaryHeap;
use std::cmp::Ordering;

// Struktur data Node untuk menyimpan informasi posisi, biaya, dan prioritas.
#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    position: (usize, usize), // Posisi dalam matriks (x, y).
    cost: usize,             // Biaya dari titik awal ke node ini.
    priority: usize,         // Prioritas untuk antrean (cost + heuristic).
}

// Implementasi trait Ord untuk memungkinkan Node diurutkan berdasarkan prioritas.
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // Urutan prioritas: lebih kecil lebih baik, dengan tie-break pada cost.
        other.priority.cmp(&self.priority).then_with(|| self.cost.cmp(&other.cost))
    }
}

// Implementasi PartialOrd untuk mendukung perbandingan parsial pada Node.
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Fungsi heuristic untuk menghitung estimasi jarak antara dua titik (menggunakan Manhattan distance).
fn heuristic(a: (usize, usize), b: (usize, usize)) -> usize {
    ((a.0 as isize - b.0 as isize).abs() + (a.1 as isize - b.1 as isize).abs()) as usize
}

// Implementasi algoritma A* untuk pencarian jalur terpendek.
fn a_star_search(grid: &Vec<Vec<i32>>, start: (usize, usize), goal: (usize, usize)) -> Option<Vec<(usize, usize)>> {
    // Arah gerakan: kanan, bawah, kiri, atas.
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut open_set = BinaryHeap::new(); // Antrean prioritas untuk node yang akan diproses.
    let mut came_from = vec![vec![None; grid[0].len()]; grid.len()]; // Menyimpan jejak node sebelumnya.
    let mut cost_so_far = vec![vec![usize::MAX; grid[0].len()]; grid.len()]; // Menyimpan biaya terendah ke setiap node.

    // Tambahkan node awal ke antrean.
    open_set.push(Node {
        position: start,
        cost: 0,
        priority: heuristic(start, goal),
    });
    cost_so_far[start.0][start.1] = 0;

    // Proses node dalam antrean sampai kosong atau mencapai tujuan.
    while let Some(current) = open_set.pop() {
        // Jika mencapai tujuan, bangun jalur dari belakang.
        if current.position == goal {
            let mut path = vec![];
            let mut current_pos = Some(goal);
            while let Some(pos) = current_pos {
                path.push(pos);
                current_pos = came_from[pos.0][pos.1];
            }
            path.reverse(); // Urutkan jalur dari awal ke tujuan.
            return Some(path);
        }

        // Periksa semua arah gerakan dari posisi saat ini.
        for dir in &directions {
            let new_x = current.position.0 as isize + dir.0;
            let new_y = current.position.1 as isize + dir.1;

            // Pastikan posisi baru valid dan tidak melampaui batas matriks.
            if new_x >= 0 && new_x < grid.len() as isize && new_y >= 0 && new_y < grid[0].len() as isize {
                let new_position = (new_x as usize, new_y as usize);
                // Abaikan jika posisi baru adalah rintangan.
                if grid[new_position.0][new_position.1] == 1 {
                    continue;
                }

                // Hitung biaya baru ke posisi tersebut.
                let new_cost = cost_so_far[current.position.0][current.position.1] + 1;
                // Jika biaya baru lebih rendah, perbarui jalur dan tambahkan ke antrean.
                if new_cost < cost_so_far[new_position.0][new_position.1] {
                    cost_so_far[new_position.0][new_position.1] = new_cost;
                    came_from[new_position.0][new_position.1] = Some(current.position);
                    open_set.push(Node {
                        position: new_position,
                        cost: new_cost,
                        priority: new_cost + heuristic(new_position, goal),
                    });
                }
            }
        }
    }

    // Jika tujuan tidak dapat dicapai, kembalikan None.
    None
}

fn main() {
    // Definisikan grid matriks dengan 0 sebagai jalan bebas dan 1 sebagai rintangan.
    let grid = vec![
        vec![0, 0, 0, 0, 0],
        vec![0, 1, 1, 1, 0],
        vec![0, 0, 0, 1, 0],
        vec![0, 1, 0, 0, 0],
        vec![0, 0, 0, 0, 0],
    ];

    let start = (0, 0); // Titik awal.
    let goal = (4, 4); // Titik tujuan.

    // Jalankan algoritma A* dan tampilkan hasilnya.
    match a_star_search(&grid, start, goal) {
        Some(path) => {
            println!("Jalur ditemukan:");
            for (x, y) in path {
                println!("({}, {})", x, y);
            }
        }
        None => println!("Jalur tidak ditemukan."),
    }
}
