use std::fs;

const CONFIG_FILE_NAME: &'static str = "Makefile";

fn main() {
    let file_path = CONFIG_FILE_NAME;
    let error_msg = format!("Could not read {file_path}");
    let contents = fs::read_to_string(file_path).expect(&error_msg);

    println!("{contents}");
}
