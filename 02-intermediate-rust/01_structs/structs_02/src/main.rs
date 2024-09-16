// Implement a method for the book struct to display its details.

// To get rid of warnings, so annoying.
#[allow(dead_code)]

#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    pages: u32
}

//Implemtent a method for the book struct to display its details.
impl Book {
    fn display_details(&self) {
        println!("Title: {}", self.title);
        println!("Author: {}", self.author);
        println!("Pages: {}", self.pages);
    }
}

fn main() {
    let book: Book = Book {
        title: "Programming Rust".to_string(),
        author: "Jim Blandy, Jason Orendorff and Leonora F.S. Tindall".to_string(),
        pages: 736
    };
    book.display_details();
}
