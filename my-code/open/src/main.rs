
use libc::{open, read, close, fork, O_RDONLY};
use std::ffi::CString;

fn main() {
    unsafe {
        // Prepare file name
        let filename = CString::new("file.txt").unwrap();

        // Open the file using the system call open()
        // O_RDONLY = read-only
        let fd = open(filename.as_ptr(), O_RDONLY);

        if fd < 0 {
            eprintln!("Failed to open file!");
            return;
        }

        println!("File opened with fd = {}", fd);

        // Fork
        let pid = fork();

        if pid < 0 {
            eprintln!("fork() failed!");

        } else if pid == 0 {
            // Child process
            println!("Child reading...");

            let mut buffer = [0u8; 32];
            let bytes_read = read(fd, buffer.as_mut_ptr() as *mut _, 32);

            println!(
                "Child read {} bytes: {}",
                bytes_read,
                String::from_utf8_lossy(&buffer)
            );

            close(fd);
            println!("Child closed file.");

        } else {
            // Parent process
            println!("Parent reading...");

            let mut buffer = [0u8; 32];
            let bytes_read = read(fd, buffer.as_mut_ptr() as *mut _, 32);

            println!(
                "Parent read {} bytes: {}",
                bytes_read,
                String::from_utf8_lossy(&buffer)
            );

            close(fd);
            println!("Parent closed file.");
        }
    }
}


