use nom::{
  IResult,
  sequence::delimited,
  // see the "streaming/complete" paragraph lower for an explanation of these submodules
  character::complete::char,
  bytes::complete::is_not
};

pub fn parse(input_str: &str) -> IResult<&str, &str> {
    return parens(input_str);
}

fn parens(input: &str) -> IResult<&str, &str> {
  delimited(char('('), is_not(")"), char(')'))(input)
}

fn is_select(str: &str) -> bool {
    return str == "select";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_select() {
        let input = "(hoge)";
        assert_eq!(parse(input), Ok(("", "hoge")));
    }
}
