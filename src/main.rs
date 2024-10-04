use h_data_base::{database, get_input, match_input, set_development_variables};

fn main() {
    set_development_variables();
    let mut tables: Vec<database::DataBase> = Vec::new();
    loop {
        match_input(get_input("Enter command"), &mut tables)
    }
}
