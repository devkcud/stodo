mod utils;
use utils::{ config::init_config, todoman::get_todo_list };

use crate::utils::todoman::get_todo_by_id;

fn main() {
    let config = init_config();

    println!("{}", get_todo_list(&config));
    println!("{}", get_todo_by_id(&config, 0).expect(""));
}