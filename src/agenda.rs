use components::task::AgendaPage;

mod components {
    pub mod task;
}
mod enums {
    pub mod status;
}


pub struct Agenda {
    tasks: Vec<AgendaPage>,
}

impl Agenda {
    pub fn add(&mut self, title: String) {
        println!("Adding task: {}", title);
        self.tasks.push(AgendaPage::new(title));
    }
    pub fn list_all(&self) -> &Vec<AgendaPage>{
        return &self.tasks;
    }
    
    pub fn find_by_title(&self, input_user: &String) -> Option<&AgendaPage> {
        for page in &self.tasks {
            if page.title.contains(input_user) { 
                return Some(page);
            }
        }
        return None;
    }


    pub(crate) fn new() -> Self {
        Self { 
            tasks: Vec::new(),
        }
    }
}