use std::fs::{ read_to_string, write, OpenOptions };
use std::io::prelude::*;
use std::ops::Sub;

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

pub fn add_todo(config: &Configuration, todo: String) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&config.todo_list_file)
        .unwrap();

    writeln!(file, "{}", todo).expect(
        format!("Could not write file: {}", &config.todo_list_file).as_str()
    );
}

pub fn remove_todo(config: &Configuration, id: usize) {
    let todo_list = get_todo_list(&config);

    if id > todo_list.lines().count().sub(1) {
        return ();
    }

    let mut lines = todo_list.lines().collect::<Vec<&str>>();
    lines.remove(id);

    write(&config.todo_list_file, lines.join("\n")).unwrap();
}

pub fn clear_todo_list(config: &Configuration) {
    write(&config.todo_list_file, "").expect(
        format!("Could not write file: {}", &config.todo_list_file).as_str()
    );
}