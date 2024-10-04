use std::{env, fs};
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Write;

pub struct DataBase {
    name: String,
    rows: Vec<(String, String)>, // (name, type)
    row_size: usize,
}
impl DataBase {
    pub fn new(name: &str, rows: &str) -> DataBase {
        let (rows_of_table, row_size) = Self::create_files(name, rows);
        DataBase {
            name: name.to_string(),
            rows: rows_of_table,
            row_size,
        }
    }
    fn create_files(name: &str, rows: &str) -> (Vec<(String, String)>, usize) {
        let mut rows_of_table: Vec<(String, String)> = Vec::new();
        let database_dir = env::var("DATABASE_DIR").unwrap();
        let directory_path = database_dir + "/" + name + "/";
        fs::create_dir_all(directory_path.clone()).unwrap();
        let row_size = Self::make_column_files(rows, &mut rows_of_table, &directory_path);
        File::create(directory_path.clone() + "table").unwrap();
        let mut file_of_rows = File::create(directory_path.clone() + "rows").unwrap();
        file_of_rows.write_all(rows.as_bytes()).unwrap();
        (rows_of_table, row_size)
    }
    fn make_column_files(
        rows: &str,
        rows_of_table: &mut Vec<(String, String)>,
        directory_path: &str,
    ) -> usize {
        let splited_rows = rows.split_ascii_whitespace();
        let mut row_size = 0;
        for row in splited_rows {
            let mut parts = row.split(",");
            if parts.clone().count() != 2 {
                panic!("wrong row format");
            }
            let row_name = parts.next().unwrap().to_string();
            let row_type = parts.next().unwrap().to_string();
            row_size = Self::get_row_size(row_type.as_str());
            rows_of_table.push((row_name.clone(), row_type.clone()));
            let filename = directory_path.to_string() + row_name.as_str();
            File::create(filename).unwrap();
        }
        row_size
    }
    fn get_row_size(row_type: &str) -> usize {
        let mut row_size = 0;
        match row_type {
            "bool" => row_size += 1,
            "u8" => row_size = row_size + 1,
            "u16" => row_size = row_size + 2,
            "u32" => row_size = row_size + 4,
            "u64" => row_size = row_size + 8,
            "i8" => row_size = row_size + 1,
            "i16" => row_size = row_size + 2,
            "i32" => row_size = row_size + 4,
            "i64" => row_size = row_size + 8,
            "f64" => row_size = row_size + 64,
            "f32" => row_size = row_size + 32,
            "char" => row_size = row_size + 4,
            "string8" => row_size = row_size + 32,
            "string16" => row_size = row_size + 64,
            "string32" => row_size = row_size + 128,
            "string64" => row_size = row_size + 256,
            "string128" => row_size = row_size + 512,
            _ => {
                panic!("type of row not found")
            }
        }
        row_size
    }
}
impl Display for DataBase {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut string =
            self.name.clone() + " with row size of " + self.row_size.to_string().as_str() + " :";
        for row in self.rows.clone() {
            string.push_str(&*("(".to_owned() + row.0.as_str() + "," + row.1.as_str() + ") "));
        }
        write!(f, "{}", string)
    }
}
