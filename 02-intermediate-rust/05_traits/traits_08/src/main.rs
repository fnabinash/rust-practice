// Create a trait called Filterable with a method filter that filters elements from a collection of structs representing products (Product { name, price, in_stock }).

trait Filterable {
    fn filter_by_stock(&self) -> Self;
    fn filter_by_price(&self, max_price: f32) -> Self;
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
struct Product {
    name: String,
    price: f32,
    in_stock: bool
}

impl Product {
    fn new(name: &str, price: f32, in_stock: bool) -> Self {
        Self { name: name.to_string(), price, in_stock }
    }
}

impl Filterable for Vec<Product> {
    fn filter_by_stock(&self) -> Self {
        let mut filtered_products: Vec<Product> = Vec::new();

        for product in self {
            if product.in_stock {
                filtered_products.push(product.clone());
            }
        }

        filtered_products
    }

    fn filter_by_price(&self, max_price: f32) -> Self {
        let mut filtered_products: Vec<Product> = Vec::new();

        for profuct in self {
            if profuct.price < max_price {
                filtered_products.push(profuct.clone());
            }
        }

        filtered_products
    }
}

fn main() {
    let chocolate: Product = Product::new("Swiss", 12.43, true);
    let tshirt: Product = Product::new("T-Shirt", 23.54, true);
    let pen: Product = Product::new("Binc", 1.45, false);

    let all_products: Vec<Product> = vec![chocolate, tshirt, pen];

    println!("Products that are in stocks: {:?}", all_products.filter_by_stock());
    println!("Products that are under $15: {:?}", all_products.filter_by_price(15.00));
}
