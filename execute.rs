use std::collections::HashMap;

use crate::file;
use crate::check_null;

// BEGIN INTEGER SECTION
pub fn set_int(line: String) -> HashMap<String, i32> {
    let mut line_array : Vec<String> = line.split(" ").map(|s| s.to_string()).collect();

    let mut integers: HashMap<String, i32> = HashMap::new();
    
    if &line_array[0] != &"int".to_string() {
        return integers;
    }
    
    let mut chars = line_array[3].chars();
    chars.next_back();
    line_array[3] = chars.as_str().to_string();
    
    if line_array[3].contains(".") || check_null::is_int_null(line_array[3].clone().parse::<i32>().unwrap()) {
        println!("Integer {}, is not an integer", line_array[1]);
        return integers;
    }
    
    if &line_array[0] == &"int".to_string() {
        integers.insert(String::from(&line_array[1]), line_array[3].parse::<i32>().unwrap());
    }

    integers
}

pub fn get_int(line: String, integers: HashMap<String, i32>) -> i32 {
    let res = integers.get(&line.clone());
    if res == None {
        return i32::MIN;
    }
    res.unwrap().clone()
}

pub fn is_int_variable(name: String, hash: HashMap<String, i32>) -> bool {
    let res = hash.get(&name.clone());
    if res == None {
        return false;
    }
    true
}
// END INTEGER SECTION

// BEGIN STRING SECTION
pub fn set_string(line: String) -> HashMap<String, String> {
    let mut line_array : Vec<String> = line.split(" ").map(|s| s.to_string()).collect();

    let mut strings: HashMap<String, String> = HashMap::new();

    if &line_array[0] != &"string".to_string() {
        return strings;
    }
    
    let mut chars = line_array[3].chars();
    chars.next_back();
    line_array[3] = chars.as_str().to_string();
    
    if !line_array[3].contains(&'"'.to_string()) {
        println!("string {}, is not an string", line_array[1]);
        return strings;
    }
    
    if &line_array[0] == &"string".to_string() {
        line_array[3] = file::convert_string::remove_first_and_last(line_array[3].clone());
        strings.insert(String::from(&line_array[1]), line_array[3].to_string());
    }

    strings
}

pub fn get_string(line: String, strings: HashMap<String, String>) -> String {
    let res = strings.get(&line.clone());
    if res == None {
        return "Cift : String : None".to_string();
    }
    res.unwrap().clone()
}

pub fn is_string_variable(name: String, hash: HashMap<String, String>) -> bool {
    let res = hash.get(&name.clone());
    if res == None {
        return false;
    }
    true
}
