// Implement a program that creates a custom iterator for a struct representing a range of numbers.

struct Numbers {
    list: Vec<u32>,
}

impl IntoIterator for Numbers {
    type Item = u32;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.list.into_iter()
    }
}

fn main() {
    let numbers: Numbers = Numbers {
        list: vec![1, 25, 34, 7, 84, 6]
    };

    for number in numbers.into_iter() {
        println!("{}", number);
    }
}
