use std::fs::read_to_string;

use super::config::Configuration;

pub fn get_todo_list(config: &Configuration) -> String {
    return read_to_string(&config.todo_list_file)
        .expect(format!("Could not read file: {}", &config.todo_list_file).as_str())
        .trim()
        .to_string();
}

pub fn get_todo_by_id(config: &Configuration, id: usize) -> Result<String, String> {
    let todo_list = get_todo_list(&config).trim().to_string();

    if todo_list.len() == 0 {
        return Err(String::from("Todo list is empty."));
    }

    if todo_list.lines().nth(id).is_none() {
        return Err(format!("Item at index {} is null", id));
    }

    return Ok(String::from(todo_list.lines().nth(id).unwrap()));
}