// Write a program that stores student grades in a HashMap and calculates the average grade.

use std::collections::HashMap;

fn main() {
    let mut student_grades: HashMap<String, f32> = HashMap::new();

    student_grades.insert(String::from("Alice"), 85.0);
    student_grades.insert(String::from("Bob"), 78.5);
    student_grades.insert(String::from("Charlie"), 92.0);
    student_grades.insert(String::from("Diana"), 88.5);
    student_grades.insert(String::from("Eve"), 76.0);

    let average_grade = calculate_average(&student_grades);

    println!("The average grade of the students is: {:.2}", average_grade);
}

fn calculate_average(grades: &HashMap<String, f32>) -> f32 {
    let total: f32 = grades.values().sum();
    let count = grades.len();

    total / count as f32
}
