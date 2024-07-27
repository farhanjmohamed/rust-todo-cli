use std::collections::HashMap;
use std::io;

fn main() {
    println!("Todo CLI");
    let mut cmd: String = String::new();
    let mut todos = HashMap::new();

    while cmd != "q" {
        cmd = String::new();
        println!("(C)reate Todo, (R)ead Todos, (U)pdate Todo, (D)elete Todo. (Q) to quit.");
        io::stdin().read_line(&mut cmd).expect("String");

        cmd = cmd.trim().to_lowercase();
        println!("you entered {}", cmd);

        if cmd == "c" {
            println!("Enter Todo Name: ");
            let mut todo_name: String = String::new();
            io::stdin().read_line(&mut todo_name).expect("String");
            todo_name = todo_name.trim().to_string();

            println!("Enter Todo Content: ");
            let mut todo_content: String = String::new();
            io::stdin().read_line(&mut todo_content).expect("String");
            todo_content = todo_content.trim().to_string();

            println!("Create Todo");
            todos.insert(todo_name, todo_content);
        } else if cmd == "r" {
            println!("Todos:");
            for (key, value) in &todos {
                println!("{key}: {value}");
            }
            println!("Read Todo");
        } else if cmd == "u" {
            println!("_______UPDATE_TODO_______");
            println!("Search Todo: ");
            let mut searched_todo: String = String::new();
            io::stdin().read_line(&mut searched_todo).expect("String");
            searched_todo = searched_todo.trim().to_string();

            println!("{}", todos[&searched_todo]);
        } else if cmd == "d" {
            println!("Delete Todo");
        } else {
            println!("Enter Valid Command!");
        }
    }

    println!("End of program")
}
