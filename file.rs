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

pub mod convert_string {
    pub fn find_between(line: String, first: char, second: char) -> String {
        let binding : String = line.clone();
        
        let first_byte : usize = binding.find(first).unwrap() + 1;
        let second_byte : usize = binding.find(second).unwrap();
    
        let chars = line.as_bytes();
    
        let s = String::from_utf8(chars[first_byte..second_byte].to_vec()).unwrap();
    
        return s;
    }
    
    pub fn get_letter(line: String, i: usize) -> String {
        let vecstr : Vec<u8> = vec![line.as_bytes()[i]];
            
        let firstch = String::from_utf8(vecstr).unwrap();
    
        return firstch;
    }

    pub fn remove_first_and_last(slice: String) -> String {
        let mut chars = slice.chars();
        chars.next();
        chars.next_back();
        let s = chars.as_str().to_string();

        return s;
    }
}
