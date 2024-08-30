// Create a struct to represent a book with fields for title, author, and pages.

struct Book {
    title: String,
    author: String,
    pages: u32
}

fn main() {
    let book: Book = Book {
        title: "The Rust Programming Language".to_string(),
        author: "Steve Klabnik and Carol Nichols".to_string(),
        pages: 552
    };

    println!("Book Title: {:?}", book.title);
    println!("Book Author: {:?}", book.author);
    println!("Book Pages: {:?}", book.pages);
}
