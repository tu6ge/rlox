use lexer::{Token, TokenType};

mod lexer;
mod parser;

// For handling language errors
pub fn report(line: usize, message: &str) {
    let err = format!("[Line {}] Error: {}", line, message);
    eprintln!("{}", err);
}

pub fn error(token: Token, message: &str) {
    if token.ttype == TokenType::Eof {
        report(token.line, &("at end ".to_owned() + message));
    } else {
        report(
            token.line,
            &("at '".to_owned() + &token.lexeme + "'. " + message),
        );
    }
}
