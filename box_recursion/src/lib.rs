#[derive(Debug, PartialEq, Clone)]
pub enum Role {
    CEO,
    Manager,
    Worker,
}

impl From<&str> for Role {
    fn from(s: &str) -> Self {
        match s {
            "CEO" => Role::CEO,
            "Manager" => Role::Manager,
        
            _ =>  Role::Worker
        }
    }
}

#[derive(Debug)]
pub struct WorkEnvironment {
    pub grade: Link,
}

pub type Link = Option<Box< Worker>>; // Complete type alias

#[derive(Debug)]
pub struct Worker {
    pub role: Role,
    pub name: String,
    pub next: Link,
}

impl WorkEnvironment {
    pub fn new() -> Self {
      
        WorkEnvironment { grade: None }
    }

    pub fn add_worker(&mut self, name: &str, role: &str) {
        let new = Worker{ name: name.to_string(), role : Role::from(role) , next: self.grade.take()} ; 
        self.grade = Some(Box::new(new));

      
    }

    pub fn remove_worker(&mut self) -> Option<String> {
        self.grade.take().map(|boxed| {
            let worker = *boxed; 
            self.grade = worker.next;  
            worker.name
        })

    }

    pub fn last_worker(&self) -> Option<(String, Role)> {
        self.grade.as_ref().map(|w| (w.name.clone(),w.role.clone() ))
     
    }
}