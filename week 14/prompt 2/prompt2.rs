use std::io; // Digunakan untuk membaca input dari pengguna.

fn main() {
    // Definisikan grid matriks dengan 0 sebagai jalan bebas dan 1 sebagai rintangan.
    let grid = vec![
        vec![0, 0, 0, 0, 0],
        vec![0, 1, 1, 1, 0],
        vec![0, 0, 0, 1, 0],
        vec![0, 1, 0, 0, 0],
        vec![0, 0, 0, 0, 0],
    ];

    let mut robot_position = (0_usize, 0_usize); // Posisi awal robot.

    loop {
        println!("Posisi robot saat ini: ({}, {})", robot_position.0, robot_position.1);
        println!("Masukkan arah gerakan (w: atas, s: bawah, a: kiri, d: kanan) atau q untuk keluar:");

        // Baca input dari pengguna.
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Gagal membaca input");
        let input = input.trim();

        // Tangani input untuk menggerakkan robot.
        let new_position = match input {
            "w" => (robot_position.0.saturating_sub(1), robot_position.1), // Gerak ke atas.
            "s" => (robot_position.0 + 1, robot_position.1),               // Gerak ke bawah.
            "a" => (robot_position.0, robot_position.1.saturating_sub(1)), // Gerak ke kiri.
            "d" => (robot_position.0, robot_position.1 + 1),               // Gerak ke kanan.
            "q" => {
                println!("Keluar dari program.");
                break;
            }
            _ => {
                println!("Input tidak valid, coba lagi.");
                continue;
            }
        };

        // Pastikan posisi baru berada dalam batas grid dan bukan rintangan.
        if new_position.0 < grid.len()
            && new_position.1 < grid[0].len()
            && grid[new_position.0][new_position.1] == 0
        {
            robot_position = new_position; // Perbarui posisi robot.
        } else {
            println!("Gerakan tidak valid: posisi di luar batas atau menabrak rintangan.");
        }
    }
}
