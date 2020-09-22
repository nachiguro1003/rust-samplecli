fn get_int_from_file() -> Result<i32,String> {
    let path = "input.txt";
    let num_str = std::fs::read_to_string(path).map_err(|err| err.to_string())?;
    num_str.trim().parse::<i32>().map(|t| t * 2).map_err(|err| err.to_string())
}

fn main() {
    match get_int_from_file() {
        Ok(x) => println!("{}",x),
        Err(e) => println!("{}",e),
    }
}
