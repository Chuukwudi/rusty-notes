enum FileSize {
    Bytes(u64),
    Kilobytes(u64),
    Megabytes(u64),
    Gigabytes(u64),
}

// fn format_size(size: u64) -> String {
//     let filesize = match size {
//         0..=999 => FileSize::Bytes(size),
//         1000..=999_999 => FileSize::Kilobytes(size / 1000),
//         1_000_000..=999_999_999 => FileSize::Megabytes(size / 1_000_000),
//         _ => FileSize::Gigabytes(size / 1_000_000_000),
//     };
    
//     match filesize {
//         FileSize::Bytes(bytes) => format!("{} bytes", bytes),
//         FileSize::Kilobytes(kb) => format!("{:.2} KB", kb as f64 / 1000.0),
//         FileSize::Megabytes(mb) => format!("{:.2} MB", mb as f64 / 1000.0),
//         FileSize::Gigabytes(gb) => format!("{:.2} GB", gb as f64 / 1000.0),
//     }
// }

// fn main() {
//     let size = 1288834567890; // Example size in bytes
//     let formatted_size = format_size(size);
//     println!("Formatted size: {}", formatted_size);
// }

impl FileSize {
    fn format_size(&self) -> String {
        match self {
            FileSize::Bytes(bytes) => format!("{} bytes", bytes),
            FileSize::Kilobytes(kb) => format!("{:.2} KB", *kb as f64 / 1000.0),
            FileSize::Megabytes(mb) => format!("{:.2} MB", *mb as f64 / 1000.0),
            FileSize::Gigabytes(gb) => format!("{:.2} GB", *gb as f64 / 1000.0),
        }
    }
}

fn main() {
    let size = 2_000_000_000_000;
    let filesize = match size {
        0..=999 => FileSize::Bytes(size),
        1000..=999_999 => FileSize::Kilobytes(size / 1000),
        1_000_000..=999_999_999 => FileSize::Megabytes(size / 1_000_000),
        _ => FileSize::Gigabytes(size / 1_000_000_000),
    };
    
    println!("File size: {}", filesize.format_size());
}

// This code shows an implementation block for the FileSize enum, 
// adding a method called format_size that returns a human-readable string representation of the file size.
// The implementation moves the formatting logic from the previous standalone function 
// into a method on the enum itself, which is a more idiomatic Rust approach. The method uses &self 
// to reference the current enum instance without taking ownership.
// In the main function, there's the beginning of a match expression similar to what we 
// saw in the previous code, but it appears to be incomplete in the image. 
// The complete code would likely convert the raw size value into the appropriate FileSize variant as we saw before.