// Implement a program that uses an enum to represent the status of an order (Pending, Shipped, Delivered) and use pattern matching to update the status.

struct Product {
    status: Status
}

enum Status {
    Pending,
    Shipped,
    Delivered
}

impl Product {
    fn update_status(&mut self) {
        match self.status {
            Status::Pending => {
                self.status = Status::Shipped;
                println!("Your product is shipped and ready to deliver.");
            },

            Status::Shipped => {
                self.status = Status::Delivered;
                println!("Your product is delivered. Enjoy!");
            },

            Status::Delivered => println!("Your product already delivered!"),
        }
    }
}

fn main() {
    let mut product: Product = Product { status: Status::Pending };
    product.update_status();
    std::thread::sleep(std::time::Duration::from_secs(2));

    product.update_status();
    std::thread::sleep(std::time::Duration::from_secs(2));

    product.update_status();
}
