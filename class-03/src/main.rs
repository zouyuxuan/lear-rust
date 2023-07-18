struct Student {
    name: String,
    age: u32,
    grade: String,
}

struct StudentManagementSystem {
    students: Vec<Student>,
}

impl StudentManagementSystem {
    fn new() -> Self {
        StudentManagementSystem {
            students: Vec::new(),
        }
    }

    fn add_student(&mut self, name: String, age: u32, grade: String) {
        let student = Student { name, age, grade };
        self.students.push(student);
    }

    fn display_students(&self) {
        for student in &self.students {
            println!("Name: {}, Age: {}, Grade: {}", student.name, student.age, student.grade);
        }
    }

    fn search_student(&self, name: &str) {
        let mut found = false;
        for student in &self.students {
            if student.name == name {
                println!("Name: {}, Age: {}, Grade: {}", student.name, student.age, student.grade);
                found = true;
                break;
            }
        }
        if !found {
            println!("Student not found.");
        }
    }
}

fn main() {
    let mut system = StudentManagementSystem::new();

    // 添加学生
    system.add_student("John".to_string(), 18, "Grade 12".to_string());
    system.add_student("Alice".to_string(), 17, "Grade 11".to_string());
    system.add_student("Bob".to_string(), 16, "Grade 10".to_string());

    // 显示学生列表
    println!("--- Student List ---");
    system.display_students();

    // 搜索学生信息
    println!("--- Search Student ---");
    system.search_student("Alice");
    system.search_student("Eve");
}