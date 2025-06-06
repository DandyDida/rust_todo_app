# 📌 Rust Todo CLI App

Aplikasi CLI sederhana dengan Rust: menyimpan dan menampilkan daftar tugas. Belajar Rust di IntelliJ IDEA.

## 🧰 Prasyarat

- [Rust](https://www.rust-lang.org/tools/install)
- [IntelliJ IDEA](https://www.jetbrains.com/idea/)
- Plugin Rust (diinstal lewat Preferences > Plugins > Rust)

## 🛠️ Langkah-langkah Setup

1. **Buka IntelliJ IDEA**  
   - File > New > Project > Rust  
   - Beri nama: `rust_todo_app`

2. **Isi `Cargo.toml`**
```toml
[package]
name = "rust_todo_app"
version = "0.1.0"
edition = "2021"

[dependencies]
```

3. **Kode di `src/main.rs`**
(Lihat file `src/main.rs` di dalam proyek ini)

4. **Jalankan**
   - Klik ▶️ di IntelliJ atau tekan `Shift + F10`

## 📂 File Output

- `todo.txt` menyimpan semua tugas.
- File ini akan otomatis dibuat jika belum ada.

## Cocok untuk:
- Pemula Rust
- Latihan I/O File
- Belajar CLI Interaction


---

## 🆕 Update Fitur

Aplikasi ini sekarang memiliki fitur:
- Edit tugas
- Hapus tugas
- Hapus semua tugas
- Struktur kode lebih modular

Menu baru:
```
1. Tambah tugas
2. Lihat semua tugas
3. Edit tugas
4. Hapus tugas
5. Hapus semua tugas
6. Keluar
```
