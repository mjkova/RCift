use std::collections::HashMap;

use std::collections::HashMap;

pub fn set_ints(line: String) -> HashMap<String, i32> {
    let mut line_array : Vec<String> = line.split(" ").map(|s| s.to_string()).collect();

    let mut integers: HashMap<String, i32> = HashMap::new();
    
    let mut chars = line_array[3].chars();
    chars.next_back();
    line_array[3] = chars.as_str().to_string();

    
    
    if line_array[3].contains(".") {
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
    if res != None {
        return ;
    }
    println!("{}", res);
    res
} fn set_ints(line: String) -> HashMap<String, i32> {
    let mut line_array : Vec<String> = line.split(" ").map(|s| s.to_string()).collect();

    let mut integers: HashMap<String, i32> = HashMap::new();
    
    let mut chars = line_array[3].chars();
    chars.next_back();
    line_array[3] = chars.as_str().to_string();
    
    if &line_array[0] == &"int".to_string() {
        integers.insert(String::from(&line_array[1]), line_array[3].parse::<i32>().unwrap());
    }

    integers
}

pub fn get_int(line: String, integers: HashMap<String, i32>) -> i32 {
    return integers.get(&line.clone()).unwrap_or(&0).clone();
} fn execute(line: String) {
    let line_array : Vec<String> = line.split(" ").map(|s| s.to_string()).collect();

    if line_array[2] == "=" {
        println!("{}", line_array[3]);
    }
}
