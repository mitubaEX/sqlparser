mod sqlparser;

fn main() {
    let input = "(hoge)";
    let result = sqlparser::parse(input);
    match result {
        Ok((_input, output)) => println!("output: {}", output),
        Err(e) => println!("has_error: {}", e),
    }
}
