mod sqlparser;

fn main() {
    let input = "select * from hoge;";
    println!("{}", sqlparser::parse(input));
}
