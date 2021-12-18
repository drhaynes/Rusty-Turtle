enum ParseError {
    GeneralError,
}

enum AST {
    Token,
}

trait Parser {
    fn parse(string: &str) -> Result<AST, ParseError> {
        unimplemented!();
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
