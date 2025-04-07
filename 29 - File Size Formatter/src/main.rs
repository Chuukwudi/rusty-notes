use std::env;

struct FileSize {
    size: f64,
    unit: String,
}
impl FileSize {
    fn new(size: f64, unit: String) -> Self {
        FileSize { size, unit }
    }

    fn to_bytes(&self) -> f64 {
        // Convert the size to bytes based on the unit
        let size = match self.unit.as_str() {
            "B" =>  self.size,
            "KB" => self.size * 1000.0,
            "MB" => self.size * 1_000_000.0,
            "GB" => self.size * 1_000_000_000.0,
            _ => panic!("Invalid unit"),
        };
        size
    }

    fn format_bytes(&self) -> String {
        // Convert the size to bytes based on the unit
        let bytes = &self.to_bytes();
        let kb = bytes / 1000.0;
        let mb = kb / 1000.0;
        let gb = mb / 1000.0;
        
        format!("Sizes {{ \"{} bytes\", \"{} kilobytes\", \"{} megabytes\", \"{} gigabytes\" }}", bytes, kb, mb, gb)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let size = &args[1];
    let numeric_and_unit: Vec<&str> = size.split(" ").collect();
    if numeric_and_unit.len() != 2 {
        println!("Please provide a size in the format <number> <unit>");
        return;
    }
    println!("File size: {:?}", numeric_and_unit);
    let size = numeric_and_unit[0].parse::<f64>().unwrap();
    let unit = numeric_and_unit[1].to_uppercase();
    let file_size = FileSize::new(size, unit);
    let bytes = file_size.to_bytes();
    println!("File size in bytes: {}", bytes);
    println!("{}", file_size.format_bytes());
    }

// cargo run -- "24 MB"