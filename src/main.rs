use std::{collections::HashMap, io};

#[derive(Debug)]
enum InputCommand {
    InsertPeople { department: String, name: String },
    PrintDepartmentPeople { department: String },
    PrintAllPeople,
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
            InputCommand::PrintDepartmentPeople { department } => {
                self.print_department_employee(department)
            }
            InputCommand::PrintAllPeople => self.print_company_employee(),
            InputCommand::UnknownCommand => {
                println!("Your command sucks, try it again")
            }
        }
    }

    fn add_employee_to_department(&mut self, department: String, employee_name: String) {
        let department = self.department.entry(department).or_insert(vec![]);
        department.push(employee_name);
        department.sort();
    }

    fn print_company_employee(&self) {
        for dep in self.department.iter() {
            println!("{}: {}", dep.0, dep.1.join(", "))
        }
    }

    fn print_department_employee(&self, department_name: String) {
        match self.department.get(&department_name) {
            Some(people) => println!("{}: {}", department_name, people.join(", ")),
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
        let input = take_input();
        let input_command = handle_input(input);
        company.process_input(input_command);
    }
}

fn take_input() -> String {
    let mut command = String::new();
    io::stdin()
        .read_line(&mut command)
        .expect("Your input sucks, try it again");

    return command.trim().to_string();
}

fn handle_input(command: String) -> InputCommand {
    let split_command = command.split_whitespace().collect::<Vec<&str>>();
    let is_add_people =
        *split_command.get(0).unwrap() == "Add" && *split_command.get(2).unwrap() == "to";

    if command == "Retrieve a list of all people in the company" {
        return InputCommand::PrintAllPeople;
    } else if command.starts_with("Retrieve a list of all people in") {
        return match split_command.last() {
            Some(&department) => InputCommand::PrintDepartmentPeople {
                department: String::from(department),
            },
            None => InputCommand::UnknownCommand,
        };
    } else if is_add_people {
        return InputCommand::InsertPeople {
            department: split_command.get(3).unwrap_or(&"Engineering").to_string(),
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
