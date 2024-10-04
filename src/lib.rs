pub mod database;

use std::fs::File;
use std::io::{stdin, Read};
use std::{env};

pub fn set_development_variables() {
    let var_file = File::open("default_env_vars");
    if var_file.is_ok() {
        let mut vars = String::new();
        var_file.unwrap().read_to_string(&mut vars).unwrap();
        let vars = vars.split_ascii_whitespace();
        for var in vars {
            let mut var = var.split("=");
            env::set_var(var.next().unwrap(), var.next().unwrap());
        }
    }
}
pub fn get_input(name_of_input: &str) -> String {
    println!("Enter {name_of_input}:");
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
pub fn match_input(input: String, tables: &mut Vec<database::DataBase>) {
    match input.as_str() {
        "new" => {
            let new_table = database::DataBase::new(get_input("name").as_str(), get_input("rows").as_str());
            tables.push(new_table);
        }
        "list" => {
            for table in tables {
                println!("{table}");
            }
        }
        _ => {
            println!("command \"{input}\" not found");
        }
    }
}