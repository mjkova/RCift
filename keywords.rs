use std::collections::HashMap;

mod convert;
use crate::execute;

pub fn get_and_execute(line: String, hash: HashMap<String, i32>, strings: HashMap<String, String>) -> bool {
    if line.contains("print") {
        return print_parse(line, hash, strings);
    }
    return false;
}

// BEGIN KEYWORD PRINT
fn print_parse(line: String, hash: HashMap<String, i32>, strings: HashMap<String, String>) -> bool {
    let slice = convert::convert_string::find_between(line.clone(), '(', ')');
        
    let firstch = convert::convert_string::get_letter(slice.clone(), 0);

    if execute::is_int_variable(slice.clone().to_string(), hash.clone()) {
        print(execute::get_int(slice.clone().to_string(), hash.clone()).to_string());
        return false;
    }
    if execute::is_string_variable(slice.clone().to_string(), strings.clone()) {
        print(execute::get_string(slice.clone().to_string(), strings.clone()).to_string());
        return false;
    }
    if firstch == '"'.to_string() {
        let s = convert::convert_string::remove_first_and_last(slice);
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

    return false;
}

fn print(msg: String) {
    println!("{}", msg);
}
// END KEYWORD PRINT
