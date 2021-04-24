pub fn parse(input_str: &str) -> &str {
    return input_str;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_select() {
        let input = "select * from hoge;";
        assert_eq!(parse(input), input);
    }
}
