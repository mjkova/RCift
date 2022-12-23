use std::env::args;
mod file;

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

    println!("{}", file::get_line(0, file_path));
}
