use agenda::Agenda;
use std::io::{stdin};
mod agenda;

fn main() {
    println!("Hello, this is your agenda!");
    let mut current_agenda = Agenda::new();
    let mut opt_selected = 0;

    while opt_selected != 4 {
        println!("- *** -");
        opt_selected = menu();       
        
        match opt_selected {
            1 => add(&mut current_agenda),
            2 => list(&current_agenda),
            3 => search(&current_agenda),
            4 => print!("great bye!"),
            _ => println!("Select an option please.")
        }
    }
 }

 fn add(agenda: &mut Agenda) {
    let mut title = String::new();
    println!("- Add the title -");
    stdin()
    .read_line(&mut title)
    .expect("Error to readin at add task");

    agenda.add(title);
    println!("- Task added!");
 }

 fn list(agenda: &Agenda) {
    for page in agenda.list_all() {
        println!(" * {}", page.title);
    }
 }

 fn search(agenda: &Agenda){
    let mut title = String::new();
    println!("- Write the title u want search -");
    stdin()
        .read_line(&mut title)
        .expect("Error at search");

    let found = agenda
        .find_by_title(&title)
        .expect("Not found any title match");

    println!("- Item found by seach: {}", found.title);
 }

 fn menu() -> u8{
    let options = [
        "1. Add new task",
        "2. List all task",
        "3. Search a task by title",
        "4. Exit"
    ];
    for opt in options {
        println!("{}", opt);
    }
    
    println!("Choose an option: ");

    let mut _user_input = String::new();
    stdin()
        .read_line(&mut _user_input)
        .expect("Error reading an option");

    return _user_input
        .trim()
        .parse()
        .expect("Error at parse");
 }