use std::env::args;
use std::fs;

fn main() { 
    let args : Vec<String> = args().collect();
    
    let mut arg_one : bool = false;
    let mut counter : i32 = 0;

    for _item in &args {
        if counter == 1 {
            arg_one = true;
        }
        counter += 1;
    }

    if !arg_one {
        println!("No cift file provided");
        return;
    }

    let file_path : String = args[1].clone();

    if !file_path.ends_with(".cift") {
        println!("Given file is not a valid cift file");
        return;
    }

    println!("{}", get_line(0, file_path));
}

fn get_line(line: i32, path: String) -> String {
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
    
    return "Could not find line".to_string();
}
