mod smart;
use smart::{ClassManagement, Student, StudentStatus};

fn main() {
    let mut class = ClassManagement::new();

    class.register_student("Alice".to_string(), "A".to_string());
    class.register_student("Bob".to_string(), "B".to_string());

    let students = class.get_all_students();
    for student in students {
        println!("Name: {}, Grade: {:?}", student.name, student.grade);
    }
}
