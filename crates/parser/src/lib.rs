use lyn::Scanner;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    EndOfLine,
    Character(usize),
}

enum AST {
    Token,
}

trait Parser {
    fn parse(string: &str) -> Result<AST, ParseError>;
}

struct LogoParser {}

impl Parser for LogoParser {
    fn parse(input: &str) -> Result<AST, ParseError> {
        Scanner::new(input);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
