use std::{ fs::{ File, create_dir_all, read_to_string }, path::Path };

use dirs::config_dir;

fn main() {
    let todos = load_todo_list();
    println!("{}", todos);
}

fn load_todo_list() -> String {
    let folder_path: String = format!("{}/stodo", config_dir().unwrap().to_str().unwrap());
    let file_path: String = format!("{}/todos.txt", &folder_path);

    create_dir_all(&folder_path).expect("Could not create todo folder.");

    if !Path::new(&file_path).is_file() {
        File::create(&file_path).expect("Could not create todo file.");
    }

    return read_to_string(&file_path).expect("Could not read todo file.").trim().to_string();
}