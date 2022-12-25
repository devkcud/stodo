use std::{ path::Path, fs::{ create_dir_all, File } };

pub struct Configuration {
    pub root_dir: String,
    pub config_file: String,
    pub todo_list_file: String,
}

pub fn init_config() -> Configuration {
    let root_dir = format!("{}/stodo", dirs::config_dir().unwrap().to_str().unwrap());

    let config = Configuration {
        root_dir: format!("{}", root_dir),
        config_file: format!("{}/config.toml", root_dir),
        todo_list_file: format!("{}/todo_list.txt", root_dir),
    };

    if !Path::new(&config.root_dir).is_dir() {
        create_dir_all(&config.root_dir).expect(
            format!("Could not create root folder: {}", &config.root_dir).as_str()
        );
    }

    if !Path::new(&config.config_file).is_file() {
        File::create(&config.config_file).expect(
            format!("Could not create config file: {}", &config.config_file).as_str()
        );
    }

    if !Path::new(&config.todo_list_file).is_file() {
        File::create(&config.todo_list_file).expect(
            format!("Could not create todos file: {}", &config.config_file).as_str()
        );
    }

    return config;
}