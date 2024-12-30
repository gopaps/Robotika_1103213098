use std::collections::VecDeque; // Digunakan untuk antrean BFS.

// Fungsi untuk mencari jalur menggunakan Breadth-First Search (BFS).
fn find_path(grid: &Vec<Vec<i32>>, start: (usize, usize), goal: (usize, usize)) -> Option<Vec<(usize, usize)>> {
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)]; // Gerakan: kanan, bawah, kiri, atas.
    let mut queue = VecDeque::new();
    let mut came_from = vec![vec![None; grid[0].len()]; grid.len()]; // Melacak jalur yang telah dilalui.

    queue.push_back(start);
    came_from[start.0][start.1] = Some(start);

    while let Some(current) = queue.pop_front() {
        if current == goal {
            let mut path = vec![];
            let mut current_pos = Some(goal);
            // Rekonstruksi jalur dari tujuan ke awal.
            while let Some(pos) = current_pos {
                path.push(pos);
                current_pos = came_from[pos.0][pos.1];
                if current_pos == Some(start) {
                    path.push(start);
                    break;
                }
            }
            path.reverse(); // Urutkan jalur dari awal ke tujuan.
            return Some(path);
        }

        for dir in &directions {
            let new_x = current.0 as isize + dir.0;
            let new_y = current.1 as isize + dir.1;

            if new_x >= 0
                && new_x < grid.len() as isize
                && new_y >= 0
                && new_y < grid[0].len() as isize
                && grid[new_x as usize][new_y as usize] == 0
                && came_from[new_x as usize][new_y as usize].is_none()
            {
                let next = (new_x as usize, new_y as usize);
                queue.push_back(next);
                came_from[next.0][next.1] = Some(current);
            }
        }
    }

    None // Kembalikan None jika tidak ada jalur ditemukan.
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

    let start = (0, 0); // Titik awal robot.
    let goal = (4, 4); // Titik tujuan robot.

    match find_path(&grid, start, goal) {
        Some(path) => {
            println!("Jalur ditemukan:");
            for step in &path {
                println!("({}, {})", step.0, step.1);
            }
            println!("Total langkah: {}", path.len() - 1);
        }
        None => println!("Tidak ada jalur yang ditemukan."),
    }
}
