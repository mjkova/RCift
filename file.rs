use std::fs;

pub fn get_line(line: i32, path: String) -> String {
    let split_char : String = "\n".to_string();
    
    let contents = fs::read_to_string(path)
        .expect("File is not readable, try checking permissions");

    let iter_for_lines = contents.split(&split_char);

    let mut counter : i32 = 0;

    for item in iter_for_lines {
        if counter == line {
            return item.parse::<String>().unwrap();
        }
        counter += 1;
    }
    
    return "Cift : String : None".to_string();
}

pub fn get_lines_count(path: String) -> i32 {
    let split_char : String = "\n".to_string();
    
    let contents = fs::read_to_string(path)
        .expect("File is not readable, try checking permissions");

    let iter_for_lines = contents.split(&split_char);

    let mut counter : i32 = 0;

    for _item in iter_for_lines {
        counter += 1;
    } 

    return counter;
}
