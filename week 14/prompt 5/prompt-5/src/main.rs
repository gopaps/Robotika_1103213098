use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// Struktur data untuk mendefinisikan event.
#[derive(Debug)]
struct Event {
    description: String, // Deskripsi event.
}

// Fungsi untuk menangani event yang dideteksi.
fn handle_event(event: &Event) {
    println!("Menangani event: {}", event.description);
    // Simulasi tindakan robot berdasarkan event.
    if event.description.contains("rintangan baru") {
        println!("Robot berhenti dan mencari jalur alternatif.");
    } else if event.description.contains("perubahan tujuan") {
        println!("Robot mengarahkan ulang ke tujuan baru.");
    } else {
        println!("Event tidak dikenali, tetap dalam mode siaga.");
    }
}

fn main() {
    // Inisialisasi queue untuk event.
    let event_queue: Arc<Mutex<VecDeque<Event>>> = Arc::new(Mutex::new(VecDeque::new()));

    // Clone Arc untuk thread event generator.
    let event_queue_producer = Arc::clone(&event_queue);

    // Thread untuk mensimulasikan deteksi event dari lingkungan.
    thread::spawn(move || {
        let simulated_events = vec![
            Event {
                description: String::from("Rintangan baru terdeteksi"),
            },
            Event {
                description: String::from("Perubahan tujuan diterima"),
            },
        ];

        for event in simulated_events {
            println!("Event terdeteksi: {}", event.description);
            let mut queue = event_queue_producer.lock().unwrap();
            queue.push_back(event);
            thread::sleep(Duration::from_secs(2)); // Simulasi waktu antar event.
        }
    });

    // Thread utama untuk menangani event.
    loop {
        let event_opt = {
            let mut queue = event_queue.lock().unwrap();
            queue.pop_front()
        };

        if let Some(event) = event_opt {
            handle_event(&event);
        } else {
            println!("Tidak ada event. Robot dalam mode siaga.");
        }

        thread::sleep(Duration::from_secs(1)); // Simulasi waktu tunggu sebelum pengecekan berikutnya.
    }
}
