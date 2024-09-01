// Implement a struct to represent a student with fields for name, age, and grades, and calculate the average grade.

struct Student {
    name: String,
    age: u8,
    grades: Vec<u32>
}

impl Student {
    fn get_avg_grades(&self) -> u32 {
        let mut sum: u32 = 0;
        let len: u32 = self.grades.len() as u32;

        for grade in &self.grades {
            sum += grade;
        }

        sum / len
    }
}

fn main() {
    let student: Student = Student {
        name: "Kamlesh".to_string(),
        age: 13,
        grades: vec![45, 56, 78, 32, 99]
    };

    let avg_grade: u32 = student.get_avg_grades();
    println!("Average grade of {:?} of age {} is {}.", student.name, student.age, avg_grade);
}
