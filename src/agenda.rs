use std::io::stdin;
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

    pub fn edit_page(&mut self) {
        println!("- Marking page with new state:"); 
        let page = &mut self.tasks.get_mut(0).unwrap();
        let current_state = page.get_state();
        let new_state = AgendaPage::to_new_state(&current_state);

        if current_state.to_lowercase().contains("delete"){
            println!(" - Want delete page? (y/n)");
            let mut delete_page: String = String::new();
            stdin()
                .read_line(&mut delete_page)
                .expect("err at response");
            if delete_page.contains("y") {
                let _ = &mut self.tasks.remove(0);
                println!(" - Deleted page");
            }
        } else {
            page.set_state(new_state);
        }
    }


    pub(crate) fn new() -> Self {
        Self { 
            tasks: Vec::new(),
        }
    }
}