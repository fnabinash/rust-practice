# Rust Practice

This repository tracks my progress in mastering Rust, focusing on building production-grade CLI tools. The exercises are structured to cover all essential Rust concepts, from basics like variables and control flow to advanced topics like asynchronous programming and unsafe Rust.

> **I'll update this repo as I complete each exercise.**

### 1. **Basics of Rust Programming (10%)**

#### Hello, World! (0.5%)
- [ ] Print "Hello, World!" to the console.
- [ ] Modify the program to take user input and print "Hello, [name]!".
- [ ] Create a program that prints a greeting based on the time of day.
- [ ] Print a multi-line string using raw strings.
- [ ] Format a string with multiple variables and print it.
- [ ] Print a list of numbers from 1 to 10
- [ ] Print the length of a given string.
- [ ] Print the result of basic arithmetic operations.
- [ ] Print a countdown from 10 to 1
- [ ] Create a program that prints a quote using escape characters.

#### Variables and Mutability (1%)
- [ ] Swap two variables without using a third variable.
- [ ] Declare a constant and print its value.
- [ ] Create a mutable variable and change its value inside a loop.
- [ ] Shadow a variable and print both values.
- [ ] Create a program that calculates the area of a rectangle.
- [ ] Convert a string to an integer and print the result.
- [ ] Declare a variable with a tuple and access its elements.
- [ ] Create a program that doubles the value of a number using a mutable variable.
- [ ] Implement a program that changes a variable's value based on user input.
- [ ] Create a program that increments a counter variable each time a button is pressed.

#### Data Types and Casting (1.5%)
- [ ] Convert a floating-point number to an integer.
- [ ] Parse a string as a number and perform arithmetic on it.
- [ ] Convert a number from one type to another and print the result.
- [ ] Create a program that determines the type of a variable.
- [ ] Implement a program that calculates the square root of a number.
- [ ] Create a program that converts Celsius to Fahrenheit.
- [ ] Convert a number to a string and concatenate it with another string.
- [ ] Write a program that casts an integer to a character.
- [ ] Convert a number from binary to decimal.
- [ ] Create a program that calculates the area of a circle using a floating-point number.

#### Control Flow (2%)
- [ ] Implement a program that checks if a number is even or odd.
- [ ] Write a simple program that uses `if-else` to compare two numbers.
- [ ] Create a calculator using a match statement.
- [ ] Implement a program that checks whether a character is a vowel or consonant.
- [ ] Write a program that checks if a number is positive, negative, or zero.
- [ ] Implement a program that finds the largest of three numbers.
- [ ] Create a program that simulates a basic traffic light system.
- [ ] Write a program that checks if a year is a leap year.
- [ ] Create a simple menu-driven program using a loop and match statement.
- [ ] Write a program that calculates the factorial of a number using a loop.

#### Functions (2.5%)
- [ ] Create a function that returns the square of a number.
- [ ] Implement a function that takes two numbers and returns their sum.
- [ ] Write a function that checks if a number is prime.
- [ ] Implement a function that returns the nth Fibonacci number.
- [ ] Write a function that reverses a string.
- [ ] Create a function that checks if a string is a palindrome.
- [ ] Implement a function that returns the maximum of two numbers.
- [ ] Write a function that calculates the greatest common divisor (GCD) of two numbers.
- [ ] Implement a function that calculates the sum of an array of numbers.
- [ ] Create a function that generates a random number within a specified range.

#### Ownership and Borrowing (3%)
- [ ] Create a function that takes ownership of a string and prints it.
- [ ] Implement a function that borrows a string and returns its length.
- [ ] Write a function that takes two borrowed strings and concatenates them.
- [ ] Create a function that returns ownership of a string to the caller.
- [ ] Implement a function that modifies a borrowed string using mutable references.
- [ ] Write a program that demonstrates the use of multiple references.
- [ ] Create a function that borrows an array and returns its sum.
- [ ] Implement a program that shows how ownership changes when a variable is reassigned.
- [ ] Write a function that takes ownership of a vector and returns its length.
- [ ] Create a program that demonstrates the use of references in function parameters and return types.

### 2. **Intermediate Rust (20%)**

#### Structs (4%)
- [ ] Create a `struct` to represent a book with fields for title, author, and pages.
- [ ] Implement a method for the book `struct` to display its details.
- [ ] Write a program that creates a `struct` to represent a point in 2D space and calculate the distance between two points.
- [ ] Create a `struct` to represent a rectangle and calculate its area.
- [ ] Implement a `struct` to represent a student with fields for name, age, and grades, and calculate the average grade.
- [ ] Write a program that creates a `struct` to represent a complex number and implements methods for addition and subtraction.
- [ ] Create a `struct` to represent a car with fields for make, model, and year, and implement methods to start and stop the car.
- [ ] Implement a `struct` to represent a bank account with fields for account number, balance, and owner, and create methods to deposit and withdraw money.
- [ ] Write a program that creates a `struct` to represent a person with fields for name, age, and address, and implement methods to update the address.
- [ ] Create a `struct` to represent a circle with a field for radius, and implement methods to calculate the circumference and area.

#### Enums and Pattern Matching (6%)
- [ ] Create an `enum` to represent traffic light states and use pattern matching to display messages.
- [ ] Implement a program that uses an `enum` to represent different shapes (Circle, Square, Rectangle) and calculate their area.
- [ ] Write a program that uses an `enum` to represent the result of a calculation (Success, Error) and uses pattern matching to handle both cases.
- [ ] Create an `enum` to represent different types of users (Admin, Guest, Registered) and use pattern matching to display messages based on user type.
- [ ] Implement a program that uses an `enum` to represent different payment methods (CreditCard, PayPal, Cash) and calculate the total cost based on the payment method.
- [ ] Write a program that uses an `enum` to represent different file types (Text, Binary, Image) and use pattern matching to process files based on their type.
- [ ] Create an `enum` to represent different weather conditions (Sunny, Rainy, Cloudy) and use pattern matching to display appropriate messages.
- [ ] Implement a program that uses an `enum` to represent the status of an order (Pending, Shipped, Delivered) and use pattern matching to update the status.
- [ ] Write a program that uses an `enum` to represent different error types (NotFound, PermissionDenied, Unknown) and use pattern matching to handle errors.
- [ ] Create an `enum` to represent different operations (Add, Subtract, Multiply, Divide) and use pattern matching to implement a basic calculator.

#### Collections (Vectors, HashMaps) (8%)
- [ ] Create a program that adds and removes elements from a vector.
- [ ] Implement a program that finds the largest number in a vector.
- [ ] Write a program that sorts a vector of integers in ascending and descending order.
- [ ] Create a program that removes duplicate elements from a vector.
- [ ] Implement a program that uses a `HashMap` to count the occurrences of words in a text.
- [ ] Write a program that stores student grades in a `HashMap` and calculates the average grade.
- [ ] Create a program that stores user preferences in a `HashMap` and allows updating and retrieving preferences.
- [ ] Implement a program that uses a `HashMap` to group words by their first letter.
- [ ] Write a program that uses a `HashMap` to store and retrieve configuration settings.
- [ ] Create a program that uses a `HashMap` to implement a basic phone book with names and phone numbers.

#### Error Handling (10%)
- [ ] Write a program that handles file reading errors and displays an appropriate message if the file is not found.
- [ ] Implement a function that returns a `Result` type and handles both success and error cases.
- [ ] Create a program that parses a string as an integer and handles invalid input with error messages.
- [ ] Write a function that attempts to open a file and returns a custom error type if the file cannot be opened.
- [ ] Implement a program that reads user input and handles errors if the input is not a valid number.
- [ ] Create a program that uses `Option` to handle missing values and displays a default value if a value is `None`.
- [ ] Write a program that attempts to divide two numbers and handles division by zero errors.
- [ ] Implement a function that performs a calculation and returns a `Result` type with a custom error message if the calculation fails.
- [ ] Create a program that reads a configuration file and handles errors if the file is missing or corrupted.
- [ ] Write a function that attempts to connect to a server and returns a custom error type if the connection fails.

#### Traits (12%)
- [ ] Create a trait called `Printable` with a method `print` and implement it for different types (e.g., `struct`, `enum`).
- [ ] Write a program that creates a trait called `Calculable` with methods for addition, subtraction, multiplication, and division, and implement it for a `struct` representing a calculator.
- [ ] Implement a trait called `Drawable` with a method `draw` and implement it for different shapes (`Circle`, `Square`, `Rectangle`).
- [ ] Write a program that creates a trait called `Serializable` with methods for serializing and deserializing data, and implement it for a `struct` representing a user.
- [ ] Implement a trait called `Comparable` with a method `compare` and implement it for different types (`i32`, `f64`, `String`).
- [ ] Write a program that creates a trait called `Cloneable` with a method `clone` and implement it for a `struct` representing a document.
- [ ] Implement a trait called `Readable` with a method `read` and implement it for different file types (`TextFile`, `BinaryFile`, `ImageFile`).
- [ ] Write a program that creates a trait called `Savable` with a method `save` and implement it for a `struct` representing a game.
- [ ] Implement a trait called `Updatable` with a method `update` and implement it for different data types (`i32`, `f64`, `String`).
- [ ] Write a program that creates a trait called `Convertible` with methods for converting data between different formats, and implement it for a `struct` representing a currency converter.

#### Iterators and Closures (15%)
- [ ] Create a program that uses an iterator to sum all elements in a vector.
- [ ] Implement a program that filters out even numbers from a vector using an iterator.
- [ ] Write a program that maps a vector of integers to their squares using an iterator.
- [ ] Create a program that uses an iterator to find the first element in a vector that satisfies a condition.
- [ ] Implement a program that creates a custom iterator for a `struct` representing a range of numbers.
- [ ] Write a program that chains multiple iterators to process a vector of numbers.
- [ ] Create a program that uses a closure to sort a vector of integers in descending order.
- [ ] Implement a program that uses a closure to filter a vector of strings based on their length.
- [ ] Write a program that uses a closure to find the maximum value in a vector of integers.
- [ ] Create a program that uses a closure to generate a sequence of numbers and returns the result as a vector.

#### Modules and Crates (20%)
- [ ] Create a module to organize functions for mathematical operations and use it in a main program.
- [ ] Implement a program that uses a module to organize functions for file handling and use it in a main program.
- [ ] Write a program that creates a module to organize functions for string processing and use it in a main program.
- [ ] Create a module to organize functions for error handling and use it in a main program.
- [ ] Implement a program that uses a module to organize functions for working with collections (`Vector`, `HashMap`) and use it in a main program.
- [ ] Write a program that creates a module to organize functions for networking (e.g., making HTTP requests) and use it in a main program.
- [ ] Create a crate for common utilities (e.g., logging, configuration) and use it in a project.
- [ ] Implement a program that creates a module to organize functions for working with dates and times and use it in a main program.
- [ ] Write a program that creates a crate to provide a library of functions for data serialization (e.g., JSON, XML) and use it in a project.
- [ ] Create a program that uses a third-party crate (e.g., `serde`, `regex`) and organizes the code into modules.

### 3. **Advanced Rust Concepts (40%)**

#### Concurrency (25%)
- [ ] Write a program that uses threads to perform multiple calculations in parallel.
- [ ] Implement a program that spawns multiple threads to process a large array and combines the results.
- [ ] Create a program that uses channels to send messages between threads.
- [ ] Implement a program that uses a thread pool to manage and execute multiple tasks concurrently.
- [ ] Write a program that uses `Arc` (atomic reference counting) to share data between threads safely.
- [ ] Create a program that uses `Mutex` to protect shared data from race conditions.
- [ ] Implement a program that uses `RwLock` to allow multiple readers or one writer at a time.
- [ ] Write a program that uses threads to download multiple files concurrently.
- [ ] Create a program that implements a simple producer-consumer pattern using threads and channels.
- [ ] Implement a program that uses threads to simulate a basic chat server where multiple clients can send and receive messages concurrently.

#### Asynchronous Programming (28%)
- [ ] Write an asynchronous program that fetches data from a web API using `async/await`.
- [ ] Implement a program that performs multiple HTTP requests concurrently using `async/await`.
- [ ] Create an asynchronous program that reads and writes to a file without blocking the main thread.
- [ ] Write a program that uses asynchronous functions to perform a long-running task in the background.
- [ ] Implement a program that uses `tokio` or `async-std` to create an asynchronous TCP server.
- [ ] Create an asynchronous program that simulates a delay using `tokio::time::sleep` or similar.
- [ ] Write a program that performs multiple database queries concurrently using asynchronous functions.
- [ ] Implement an asynchronous program that streams data from a source and processes it in real-time.
- [ ] Create an asynchronous program that handles multiple client connections to a server concurrently.
- [ ] Write a program that uses asynchronous functions to manage a pool of resources (e.g., database connections).

#### Smart Pointers (Box, Rc, RefCell) (32%)
- [ ] Write a program that uses `Box` to create a linked list.
- [ ] Implement a program that uses `Rc` to create a tree data structure with shared ownership of nodes.
- [ ] Create a program that uses `RefCell` to modify data inside a `struct` with shared ownership.
- [ ] Write a program that uses `Box` to create a recursive data structure (e.g., a binary tree).
- [ ] Implement a program that uses `Rc` to share a configuration object between multiple threads.
- [ ] Create a program that uses `RefCell` to implement interior mutability in a `struct`.
- [ ] Write a program that uses `Rc` and `RefCell` together to create a graph data structure.
- [ ] Implement a program that uses `Box` to store a trait object and call methods on it dynamically.
- [ ] Create a program that uses `Rc` to manage a list of subscribers in a publish-subscribe pattern.
- [ ] Write a program that uses `RefCell` to dynamically update a data structure while maintaining shared ownership.

#### Macros (35%)
- [ ] Write a simple macro that prints "Hello, Macro!" to the console.
- [ ] Implement a macro that calculates the square of a number.
- [ ] Create a macro that generates getter and setter methods for a `struct`.
- [ ] Write a macro that repeats a block of code a specified number of times.
- [ ] Implement a macro that generates a match statement for an `enum` with multiple variants.
- [ ] Create a macro that generates a function to create a new instance of a `struct`.
- [ ] Write a macro that implements a trait for a `struct`.
- [ ] Implement a macro that creates a vector with a specified number of elements.
- [ ] Create a macro that performs compile-time checks on a `struct` field.
- [ ] Write a macro that generates code for a custom `Debug` implementation for a `struct`.

#### Unsafe Rust (40%)
- [ ] Write a program that uses raw pointers to access and modify memory.
- [ ] Implement a program that uses unsafe code to call a C function from Rust.
- [ ] Create a program that uses unsafe code to implement a custom memory allocator.
- [ ] Write a program that uses unsafe code to bypass Rust's borrowing rules and modify data.
- [ ] Implement a program that uses unsafe code to create a custom smart pointer.
- [ ] Create a program that uses unsafe code to implement a linked list with raw pointers.
- [ ] Write a program that uses unsafe code to optimize a performance-critical section of code.
- [ ] Implement a program that uses unsafe code to implement a custom trait for a foreign type.
- [ ] Create a program that uses unsafe code to directly access hardware registers.
- [ ] Write a program that uses unsafe code to create a low-level system library.

#### Lifetimes and Generics (45%)
- [ ] Write a program that uses lifetimes to ensure references are valid.
- [ ] Implement a function that uses generics to work with multiple data types.
- [ ] Create a program that uses lifetimes and generics together to create a flexible data structure.
- [ ] Write a function that returns a reference with a lifetime annotation.
- [ ] Implement a generic function that works with any type that implements a specific trait.
- [ ] Create a program that uses lifetimes to manage references in a data structure.
- [ ] Write a generic function that returns the larger of two values.
- [ ] Implement a program that uses lifetimes to manage the scope of a borrowed value.
- [ ] Create a program that uses generics to implement a custom collection type.
- [ ] Write a program that uses lifetimes and generics to implement a flexible and reusable function.

### 4. **Production-Grade CLI Projects (30%)**

#### Command-Line Tool (50%)
- [ ] Create a simple CLI tool that greets the user with their name.
- [ ] Implement a CLI tool that fetches weather information from an API.
- [ ] Write a CLI tool that performs basic file operations (e.g., create, read, write, delete).
- [ ] Create a CLI tool that interacts with a database to store and retrieve data.
- [ ] Implement a CLI tool that performs basic arithmetic operations with command-line arguments.
- [ ] Write a CLI tool that parses JSON data from a file and displays it in a readable format.
- [ ] Create a CLI tool that downloads files from the internet and saves them locally.
- [ ] Implement a CLI tool that converts data between different formats (e.g., CSV to JSON).
- [ ] Write a CLI tool that manages a to-do list with add, remove, and list operations.


- [ ] Create a CLI tool that interacts with a web API to send and receive data.

#### Task Automation (50%)
- [ ] Write a CLI tool that automates the process of renaming multiple files in a directory.
- [ ] Implement a CLI tool that automates the backup of files to a cloud storage service.
- [ ] Create a CLI tool that automates the process of deploying a website to a server.
- [ ] Write a CLI tool that automates the creation of boilerplate code for a new project.
- [ ] Implement a CLI tool that automates the process of compressing and decompressing files.
- [ ] Create a CLI tool that automates the process of generating documentation for a project.
- [ ] Write a CLI tool that automates the process of updating dependencies in a project.
- [ ] Implement a CLI tool that automates the testing and building of a Rust project.
- [ ] Create a CLI tool that automates the process of sending emails based on a schedule.
- [ ] Write a CLI tool that automates the process of monitoring system resources (e.g., CPU, memory usage).