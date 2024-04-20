use std::fs::OpenOptions;
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::time::Instant;

/// Measures the disk perfomance and sequential reading and writing.
///
/// This funcion writes a big file to disk and, after that, reads it, measuring the time consumed
/// by each operation. Using that information, it reports disk writing and reading speeds.
///
/// Example:
/// ```rust
/// disk();
/// ```
pub fn disk() -> io::Result<i32> {
    let n_bytes = 1024 * 1024 * 100;  // 100 MB 
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("temp_file.bin")?;
    let buffer = vec![0u8; 1024];
    // Writing test
    let start_write = Instant::now();
    for _ in 0..(n_bytes / 1024) {
        file.write(&buffer)?;
    }
    file.sync_all()?;
    let elapsed_write = start_write.elapsed();
    // Reading test
    file.seek(SeekFrom::Start(0))?;
    let start_read = Instant::now();
    let mut read_buffer = vec![0u8; 255];
    let mut total_read = 0;
    loop {
        let bytes_read = file.read(&mut read_buffer)?;
        if bytes_read == 0 {
            break;
        }
        total_read += bytes_read;
    }
    let elapsed_read = start_read.elapsed();
    // Speeds
    let write_speed = n_bytes as f64 / 1024.0 / 1024.0 / elapsed_write.as_secs_f64();
    let read_speed = total_read as f64 / 1024.0 / 1024.0 / elapsed_read.as_secs_f64();
    println!("Write speed: {:.1} MB/s", write_speed);
    println!("Read speed : {:.1} MB/s", read_speed);
    drop(file);
    std::fs::remove_file("temp_file.bin")?;
    Ok(0)
}
