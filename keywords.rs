use std::collections::HashMap;

use crate::execute;

pub fn get_and_execute(line: String, hash: HashMap<String, i32>) {
    if line.contains("print") {
        let slice = line.chars().rev().nth(2).unwrap();

        if execute::is_int_variable(slice.clone().to_string(), hash.clone()) {
            print(execute::get_int(slice.clone().to_string(), hash.clone()).to_string());
        }
    }
}

fn print(msg: String) {
    println!("{}", msg);
}
