use std::collections::HashMap;

use crate::execute;

pub fn get_and_execute(line: String, hash: HashMap<String, i32>) -> bool {
    if line.contains("print") {
        let start_bytes = line.find('(').unwrap(); 
            
        let end_bytes = line.find(')').unwrap();
        let slice = String::from_utf8((&line.as_bytes()[start_bytes+1..end_bytes]).to_vec()).unwrap();

        let vecstr : Vec<u8> = vec![line.as_bytes()[start_bytes+1]];
        
        let firstch = String::from_utf8(vecstr).unwrap();

        if execute::is_int_variable(slice.clone().to_string(), hash.clone()) {
            print(execute::get_int(slice.clone().to_string(), hash.clone()).to_string());
            return false;
        }
        if firstch == '"'.to_string() {
            let mut chars = slice.chars();
            chars.next();
            chars.next_back();
            let s = chars.as_str().to_string();

            print(s.to_string());
            return false;
        }
        else if firstch == ' '.to_string() {
            println!("Starting letter of the print statement is '{}', consider removing spaces beetween the quotations and parentheses", firstch);
            return true;
        }
        else if !execute::is_int_variable(slice.clone().to_string(), hash.clone()) {
            println!("Variable '{}', does not exist", slice);
            return true;
        }
    }
    return false;
}

fn print(msg: String) {
    println!("{}", msg);
}
