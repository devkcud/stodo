mod utils;
use utils::{ config::init_config, todoman::get_todo_list };

use crate::utils::todoman::{ get_todo_by_id, clear_todo_list, add_todo, remove_todo };

fn main() {
    let config = init_config();

    clear_todo_list(&config);
    add_todo(&config, String::from("Hello world! 0"));
    add_todo(&config, String::from("Hello world! 1"));
    add_todo(&config, String::from("Hello world! 2"));
    add_todo(&config, String::from("Hello world! 3"));
    add_todo(&config, String::from("Hello world! 4"));
    add_todo(&config, String::from("Hello world! 5"));
    add_todo(&config, String::from("Hello world! 6"));
    add_todo(&config, String::from("Hello world! 7"));
    add_todo(&config, String::from("Hello world! 8"));
    remove_todo(&config, 1);
    println!("{}\n", get_todo_list(&config));
    println!("{}", get_todo_by_id(&config, 0).expect(""));
}