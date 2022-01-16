mod lex;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

// use lyn::Scanner;
//
// #[derive(Debug, PartialEq)]
// pub enum ParseError {
//     EndOfLine,
//     Character(usize),
// }
//
// enum AST {
//     Token,
// }
//
// enum Token {
//     // Operators
//     Minus,
//     Plus,
//     Slash,
//     Star,
//
//     // Punctuation
//     LeftParen,
//     RightParen,
//     Comma,
//
//     // Drawing commands
//     Fd,
//     Forwards,
//     Bk,
//     Backwards,
//     Lt,
//     LeftTurn,
//     Rt,
//     RightTurn,
//     PenUp,
//     Pu,
//     PenDown,
//     Pd,
//     Clear,
//     Home,
//
//     // Reserved words
//     Repeat,
//     To,
//     End,
//
//     // Literals
//     Number(f32),
//     String(&str)
// }
//
// trait Parser {
//     fn parse(string: &str) -> Result<AST, ParseError>;
// }
//
// struct LogoParser {}
//
// impl Parser for LogoParser {
//     fn parse(input: &str) -> Result<AST, ParseError> {
//         let scanner = Scanner::new(input);
//         scanner.scan(|token| match token {
//
//         })
//
//     }
// }
//
