use std::collections::BTreeMap;

#[allow(clippy::new_without_default)]
pub struct School {
    data: BTreeMap<&'static str, u32>,
}

impl School {
    pub fn new() -> School {
        School {
            data: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &'static str) {
        self.data.insert(student, grade);
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut result = vec![];
        for grade in self.data.values() {
            if result.contains(grade) {
                continue;
            } else {
                result.push(grade.clone());
            }
        }

        result
    }

    pub fn grade(&self, query_grade: u32) -> Vec<String> {
        let mut result = vec![];
        for (student, grade) in &self.data {
            if *grade == query_grade {
                result.push(student.to_string());
            }
        }

        result
    }
}
