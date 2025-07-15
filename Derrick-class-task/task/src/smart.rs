pub enum Status {
    Active,
    Inactive,
}

pub struct Student {
    pub name: String,
    pub grade: String,
    pub status: Status,
}

pub struct Class {
    pub students: Vec<Student>,
}

impl Class {
    pub fn new() -> Self {
        Self {
            students: Vec::new(),
        }
    }

    pub fn register(&mut self, name: String, grade: String) {
        let student = Student {
            name,
            grade,
            status: Status::Active,
        };
        self.students.push(student);
    }

    pub fn edit(&mut self, index: usize, name: Option<String>, grade: Option<String>, status: Option<Status>) {
        if index < self.students.len() {
            if let Some(n) = name {
                self.students[index].name = n;
            }
            if let Some(g) = grade {
                self.students[index].grade = g;
            }
            if let Some(s) = status {
                self.students[index].status = s;
            }
        }
    }

    pub fn update(&mut self, index: usize, name: String, grade: String, status: Status) {
        if index < self.students.len() {
            self.students[index].name = name;
            self.students[index].grade = grade;
            self.students[index].status = status;
        }
    }

    pub fn delete(&mut self, index: usize) {
        if index < self.students.len() {
            self.students.remove(index);
        }
    }

    pub fn view(&self) -> Vec<&Student> {
        self.students.iter().collect()
    }
}
