use std::collections::HashMap;

use crate::file;
use crate::check_null;

pub fn set_all_ints(file_name: String) -> HashMap<String, i32> {
    let mut map : HashMap<String, i32> = HashMap::new();

    let lines : i32 = file::get_lines_count(file_name.clone());

    for i in 0..lines-1 {
        if !check_null::is_int_null(i) {
            let res : String = file::get_line(i, file_name.clone());
            if !check_null::is_string_null(res.clone()) {
                map.extend(set_ints(res));
            }
        }
    }

    map
}

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
    if res == None {
        return i32::MIN;
    }
    res.unwrap().clone()
}
