use std::{collections::HashMap, io};

#[derive(Debug)]
enum InputCommand {
    InsertPeople { department: String, name: String },
    RetrieveDepartmentPeople { department: String },
    RetrieveAllPeople,
    UnknownCommand,
}

struct Company {
    department: HashMap<String, Vec<String>>,
}

impl Company {
    fn process_input(&mut self, input: InputCommand) {
        match input {
            InputCommand::InsertPeople { department, name } => {
                self.add_employee_to_department(department, name)
            }
            InputCommand::RetrieveDepartmentPeople { department } => {
                self.print_department_employee_list(department)
            }
            InputCommand::RetrieveAllPeople => self.print_company_employee(),
            InputCommand::UnknownCommand => {
                println!("Your command sucks, try it again")
            }
        }
    }

    fn add_employee_to_department(&mut self, department: String, employee_name: String) {
        let department = self.department.entry(department).or_insert(vec![]);
        department.push(employee_name);
    }

    fn print_company_employee(&mut self) {
        for dep in self.department.iter_mut() {
            dep.1.sort();

            println!("{}: {}", dep.0, dep.1.join(", "))
        }
    }

    fn print_department_employee_list(&mut self, department_name: String) {
        match self.department.get_mut(&department_name) {
            Some(people) => {
                people.sort();
                println!("{}", people.join(", "))
            }
            None => println!("There are not such department"),
        };
    }
}

fn main() {
    write_prompt();

    let mut company = Company {
        department: HashMap::new(),
    };

    loop {
        let input = handle_input();
        company.process_input(input);
    }
}

fn handle_input() -> InputCommand {
    let mut command = String::new();
    match io::stdin().read_line(&mut command) {
        Ok(_) => println!("Command: {}", command),
        Err(_) => println!("Your input sucks, try it again"),
    };

    let split_command = command.split_whitespace().collect::<Vec<&str>>();
    let is_add_people = *split_command.get(0).unwrap_or(&"Nope") == "Add"
        && *split_command.get(2).unwrap_or(&"Nope") == "to";

    if command.trim() == "Retrieve a list of all people in the company" {
        return InputCommand::RetrieveAllPeople;
    } else if command.starts_with("Retrieve a list of all people in") {
        return match split_command.last() {
            Some(&department) => InputCommand::RetrieveDepartmentPeople {
                department: String::from(department),
            },
            None => InputCommand::UnknownCommand,
        };
    } else if is_add_people {
        return InputCommand::InsertPeople {
            department: split_command
                .get(3)
                .unwrap_or(&"Engineering")
                .to_string(),
            name: split_command.get(1).unwrap_or(&"Default").to_string(),
        };
    }

    return InputCommand::UnknownCommand;
}

fn write_prompt() {
    println!("Hello, and welcome to the department!");
    println!("You can use following commands to interact with the department, for example:");
    println!("  “Add Sally to Engineering” command will add Sally to the Engineering department");
    println!("  ”Retrieve a list of all people in Engineering” ");
    println!("  ”Retrieve a list of all people in the company” \n");
}
