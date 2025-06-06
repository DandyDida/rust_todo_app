use std::fs::{OpenOptions};
use std::io::{self, Write, BufRead, BufReader};
use std::fs;

fn main() {
    println!("📋 Todo List CLI App");
    loop {
        println!("\nMenu:");
        println!("1. Tambah tugas");
        println!("2. Lihat semua tugas");
        println!("3. Edit tugas");
        println!("4. Hapus tugas");
        println!("5. Hapus semua tugas");
        println!("6. Keluar");

        let mut pilihan = String::new();
        io::stdin().read_line(&mut pilihan).unwrap();

        match pilihan.trim() {
            "1" => tambah_tugas(),
            "2" => lihat_tugas(),
            "3" => edit_tugas(),
            "4" => hapus_tugas(),
            "5" => hapus_semua(),
            "6" => {
                println!("Sampai jumpa!");
                break;
            },
            _ => println!("Pilihan tidak valid!"),
        }
    }
}

fn tambah_tugas() {
    println!("Masukkan tugas baru:");
    let mut tugas = String::new();
    io::stdin().read_line(&mut tugas).unwrap();

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("todo.txt")
        .expect("Gagal membuka file");

    writeln!(file, "{}", tugas.trim()).expect("Gagal menulis ke file");
    println!("✅ Tugas ditambahkan!");
}

fn lihat_tugas() {
    let file = OpenOptions::new()
        .read(true)
        .create(true)
        .open("todo.txt")
        .expect("Gagal membuka file");

    let reader = BufReader::new(file);
    println!("\n📋 Daftar Tugas:");
    for (i, line) in reader.lines().enumerate() {
        if let Ok(tugas) = line {
            println!("{}. {}", i + 1, tugas);
        }
    }
}

fn baca_semua_tugas() -> Vec<String> {
    let file = OpenOptions::new()
        .read(true)
        .create(true)
        .open("todo.txt")
        .expect("Gagal membuka file");

    BufReader::new(file).lines().filter_map(Result::ok).collect()
}

fn simpan_tugas(daftar: &[String]) {
    let mut file = fs::File::create("todo.txt").expect("Gagal menulis file");
    for tugas in daftar {
        writeln!(file, "{}", tugas).expect("Gagal menulis");
    }
}

fn edit_tugas() {
    let mut daftar = baca_semua_tugas();
    lihat_tugas();

    println!("Masukkan nomor tugas yang ingin diedit:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    if let Ok(index) = input.trim().parse::<usize>() {
        if index == 0 || index > daftar.len() {
            println!("Nomor tidak valid.");
            return;
        }

        println!("Tugas baru:");
        let mut tugas_baru = String::new();
        io::stdin().read_line(&mut tugas_baru).unwrap();
        daftar[index - 1] = tugas_baru.trim().to_string();
        simpan_tugas(&daftar);
        println!("✅ Tugas berhasil diedit.");
    } else {
        println!("Input tidak valid.");
    }
}

fn hapus_tugas() {
    let mut daftar = baca_semua_tugas();
    lihat_tugas();

    println!("Masukkan nomor tugas yang ingin dihapus:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    if let Ok(index) = input.trim().parse::<usize>() {
        if index == 0 || index > daftar.len() {
            println!("Nomor tidak valid.");
            return;
        }

        daftar.remove(index - 1);
        simpan_tugas(&daftar);
        println!("🗑️ Tugas berhasil dihapus.");
    } else {
        println!("Input tidak valid.");
    }
}

fn hapus_semua() {
    fs::File::create("todo.txt").expect("Gagal menghapus semua tugas");
    println!("🧹 Semua tugas dihapus.");
}
