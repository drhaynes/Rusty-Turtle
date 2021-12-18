enum ParseError {
    GeneralError,
}

enum AST {
    Token,
}

trait Parser {
    fn parse(string: &str) -> Result<AST, ParseError>;
}

struct LogoParser {}

impl Parser for LogoParser {
    fn parse(string: &str) -> Result<AST, ParseError> {
        todo!()
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
