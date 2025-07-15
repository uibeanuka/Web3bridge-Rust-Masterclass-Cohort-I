#[derive(Debug, Clone, PartialEq)]
pub enum StudentStatus {
    Active,
    Inactive,
}

#[derive(Debug, Clone)]
pub struct Student {
    pub id: u32,
    pub name: String,
    pub grade: String,
    pub status: StudentStatus,
}

// Implementation
pub struct ClassManagement {
    pub students: Vec<Student>,
    next_id: u32,
}

impl ClassManagement {
    pub fn new() -> Self {
        Self {
            students: Vec::new(),
            next_id: 1,
        }
    }


    pub fn register_student(&mut self, name: String, grade: String) -> u32 {
        let student = Student {
            id: self.next_id,
            name,
            grade,
            status: StudentStatus::Active,
        };
        let id = self.next_id;
        self.students.push(student);
        self.next_id += 1;
        id
    }



    pub fn edit_student(&mut self, id: u32, name: Option<String>, grade: Option<String>, status: Option<StudentStatus>) -> bool {
        if let Some(student) = self.students.iter_mut().find(|s| s.id == id) {
            if let Some(new_name) = name {
                student.name = new_name;
            }
            if let Some(new_grade) = grade {
                student.grade = new_grade;
            }
            if let Some(new_status) = status {
                student.status = new_status;
            }
            true
        } else {
            false
        }
    }

  
    pub fn update_student(&mut self, id: u32, name: String, grade: String, status: StudentStatus) -> bool {
        if let Some(student) = self.students.iter_mut().find(|s| s.id == id) {
            student.name = name;
            student.grade = grade;
            student.status = status;
            true
        } else {
            false
        }
    }

    pub fn delete_student(&mut self, id: u32) -> bool {
        if let Some(pos) = self.students.iter().position(|s| s.id == id) {
            self.students.remove(pos);
            true
        } else {
            false
        }
    }

    
    pub fn get_all_students(&self) -> Vec<Student> {
        self.students.clone()
    }

   
    pub fn get_student_by_id(&self, id: u32) -> Option<&Student> {
        self.students.iter().find(|s| s.id == id)
    }


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_student() {
        let mut class_mgmt = ClassManagement::new();
        assert!(class_mgmt.students.len() == 0);
        
        let student_id = class_mgmt.register_student("John Doe".to_string(), "A".to_string());
        assert!(class_mgmt.students.len() == 1);
        assert!(student_id == 1);
        
        let student = &class_mgmt.students[0];
        assert_eq!(student.name, "John Doe");
        assert_eq!(student.grade, "A");
        assert_eq!(student.status, StudentStatus::Active);
    }

    #[test]
    fn test_edit_student() {
        let mut class_mgmt = ClassManagement::new();
        let student_id = class_mgmt.register_student("John Doe".to_string(), "A".to_string());
        
        let success = class_mgmt.edit_student(
            student_id,
            Some("John Smith".to_string()),
            Some("A+".to_string()),
            Some(StudentStatus::Inactive)
        );
        
        assert!(success);
        let student = &class_mgmt.students[0];
        assert_eq!(student.name, "John Smith");
        assert_eq!(student.grade, "A+");
        assert_eq!(student.status, StudentStatus::Inactive);
    }

    #[test]
    fn test_update_student() {
        let mut class_mgmt = ClassManagement::new();
        let student_id = class_mgmt.register_student("John Doe".to_string(), "A".to_string());
        
        let success = class_mgmt.update_student(
            student_id,
            "Updated Name".to_string(),
            "A+".to_string(),
            StudentStatus::Inactive
        );
        
        assert!(success);
        let student = &class_mgmt.students[0];
        assert_eq!(student.name, "Updated Name");
        assert_eq!(student.grade, "A+");
        assert_eq!(student.status, StudentStatus::Inactive);
    }

    #[test]
    fn test_delete_student() {
        let mut class_mgmt = ClassManagement::new();
        let student_id = class_mgmt.register_student("John Doe".to_string(), "A".to_string());
        
        assert!(class_mgmt.students.len() == 1);
        
        let success = class_mgmt.delete_student(student_id);
        assert!(success);
        assert!(class_mgmt.students.len() == 0);
        
   
        let fail = class_mgmt.delete_student(999);
        assert!(!fail);
    }

    #[test]
    fn test_view_functions() {
        let mut class_mgmt = ClassManagement::new();
        
        class_mgmt.register_student("Alice".to_string(), "A".to_string());
        class_mgmt.register_student("Bob".to_string(), "B".to_string());
        let charlie_id = class_mgmt.register_student("Charlie".to_string(), "A".to_string());
        
   
        class_mgmt.edit_student(charlie_id, None, None, Some(StudentStatus::Inactive));
        
        
        let all_students = class_mgmt.get_all_students();
        assert_eq!(all_students.len(), 3);
        
      
        let student = class_mgmt.get_student_by_id(1);
        assert!(student.is_some());
        assert_eq!(student.unwrap().name, "Alice");
    }
}