use std::collections::BinaryHeap;
use std::cmp::Ordering;

// Struktur data untuk tugas robot.
#[derive(Eq, PartialEq)]
struct Task {
    priority: u32, // Prioritas tugas (semakin besar semakin tinggi).
    description: String, // Deskripsi tugas.
}

// Implementasi Ord untuk mengurutkan berdasarkan prioritas tertinggi.
impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        other.priority.cmp(&self.priority) // Urutkan secara descending berdasarkan prioritas.
    }
}

// Implementasi PartialOrd untuk mendukung perbandingan parsial.
impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    // Inisialisasi antrean prioritas untuk tugas-tugas robot.
    let mut task_queue = BinaryHeap::new();

    // Tambahkan beberapa tugas ke antrean.
    task_queue.push(Task {
        priority: 3,
        description: String::from("Mengambil paket dari titik A"),
    });
    task_queue.push(Task {
        priority: 5,
        description: String::from("Mengirim paket ke titik B"),
    });
    task_queue.push(Task {
        priority: 1,
        description: String::from("Mengisi ulang baterai"),
    });

    // Proses tugas berdasarkan prioritas tertinggi.
    println!("Memulai eksekusi tugas robot:");
    while let Some(task) = task_queue.pop() {
        println!("Mengerjakan tugas: {} (Prioritas: {})", task.description, task.priority);
    }
}
