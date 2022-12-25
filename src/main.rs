use std::env::args;
use std::collections::HashMap;
use std::time::{Instant};
mod file;
mod execute;
mod check_null;
mod keywords;

fn main() { 
    let current = Instant::now();
    
    let mut integers : HashMap<String, i32> = HashMap::new();
    let mut strings : HashMap<String, String> = HashMap::new();
    
    let args : String = args().nth(1).expect("No cift file provided");

    let file_path : String = args.clone();

    if !file_path.ends_with(".cift") {
        println!("Given file is not a valid cift file");
        return;
    }

    let count : i32 = file::get_lines_count(file_path.clone());
    
    for i in 0..count {
        let line : String = file::get_line(i, file_path.clone());
        let execute_result : bool = keywords::get_and_execute(line.clone(), integers.clone(), strings.clone());

        integers.extend(execute::set_int(line.clone()));
        strings.extend(execute::set_string(line.clone()));

        if execute_result {
            println!("Fix errors then re-build program.");
            return;
        }
    }

    println!("Execution finished in: {:?}", current.elapsed());
}
