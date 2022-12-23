pub fn execute(line: String) {
    let line_array : Vec<String> = line.split(" ").map(|s| s.to_string()).collect();

    if line_array[2] == "=" {
        println!("{}", line_array[3]);
    }
}
