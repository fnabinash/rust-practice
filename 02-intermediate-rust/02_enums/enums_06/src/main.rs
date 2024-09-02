// Write a program that uses an enum to represent different file types (Text, Binary, Image) and use pattern matching to process files based on their type.

enum FileType {
    Text(String),
    Binary(Vec<u8>),
    Image { width: u32, height: u32 },
}

fn process_file(file: FileType) {
    match file {
        FileType::Text(content) => {
            println!("Processing text file with content: {}", content);
        }

        FileType::Binary(data) => {
            println!("Processing binary file with {} bytes.", data.len());
        }

        FileType::Image { width, height } => {
            println!("Processing image with dimensions: {}x{}", width, height);
        }
    }
}

fn main() {
    let text_file = FileType::Text(String::from("Hello, world!"));
    let binary_file = FileType::Binary(vec![0xDE, 0xAD, 0xBE, 0xEF]);
    let image_file = FileType::Image { width: 1920, height: 1080 };

    process_file(text_file);
    process_file(binary_file);
    process_file(image_file);
}
