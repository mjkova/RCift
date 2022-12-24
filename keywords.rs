use std::collections::HashMap;

use crate::execute;

pub fn get_and_execute(line: String, hash: HashMap<String, i32>) {
    if line.contains("print") {
        let slice = line.chars().rev().nth(2).unwrap();

        if execute::is_int_variable(slice.clone().to_string(), hash.clone()) {
            print(execute::get_int(slice.clone().to_string(), hash.clone()).to_string());
        }
        if slice == '"' {
            let start_bytes = line.find('(').unwrap(); 
            
            let end_bytes = line.find(')').unwrap();
            let result = &line[start_bytes+2..end_bytes-1];

            print(result.to_string());
        }
    }
}

fn print(msg: String) {
    println!("{}", msg);
}
