use std::{ fs::{ File, create_dir_all, read_to_string }, path::Path };

use dirs::config_dir;

fn main() {
    let todos: String = get_todos();
    println!("{}", get_todo_by_id(&todos, 3).unwrap_or(String::new()));
}

fn get_todos() -> String {
    let folder_path: String = format!("{}/stodo", config_dir().unwrap().to_str().unwrap());
    let file_path: String = format!("{}/todos.txt", &folder_path);

    create_dir_all(&folder_path).expect("Could not create todo folder.");

    if !Path::new(&file_path).is_file() {
        File::create(&file_path).expect("Could not create todo file.");
    }

    return read_to_string(&file_path).expect("Could not read todo file.").trim().to_string();
}

fn get_todo_by_id(todos: &str, id: usize) -> Result<String, String> {
    if todos.len() == 0 {
        return Err(String::from("Todo list is empty."));
    }

    if todos.lines().nth(id).is_none() {
        return Err(format!("Item at index {} is null", id));
    }

    return Ok(String::from(todos.lines().nth(id).unwrap_or("")));
}