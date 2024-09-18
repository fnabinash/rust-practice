// Implement a struct to represent a student with fields for name, age, and grades, and calculate the average grade.

struct Student {
    name: String,
    age: u8,
    grades: Vec<f32>
}

impl Student {
    fn get_avg_grades(&self) -> f32 {
        let mut sum: f32 = 0.0;
        let len = self.grades.len() as f32;

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
        grades: vec![45.0, 56.5, 78.6, 32.6, 100.0]
    };

    let avg_grade: f32 = student.get_avg_grades();
    println!("Average grade of {:?} of age {} is {}.", student.name, student.age, avg_grade);
}



