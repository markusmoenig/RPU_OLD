pub mod scanner;
pub mod context;
pub mod node;
pub mod object;

use std::ops::Deref;

use crate::prelude::*;

use self::scanner::TokenType;

pub enum ErrorType {
    Syntax,
    FileNotFound,
}

pub struct Error {
    error_type              : ErrorType,
    description             : String,
    line                    : u32,
}

impl Error {
    pub fn new(error_type: ErrorType, description: String, line: u32) -> Self {
        Self {
            error_type,
            description,
            line
        }
    }
}

struct Parser {
    current             : Token,
    previous            : Token,

    error               : Option<Error>,
}

impl Parser {

    pub fn new() -> Self {
        Self {
            current     : Token::synthetic("".to_owned()),
            previous    : Token::synthetic("".to_owned()),
            error       : None,
        }
    }
}

pub struct Compiler {
    scanner                 : Scanner,

    parser                  : Parser,
}

impl Compiler {

    pub fn new() -> Self {
        Self {
            scanner         : Scanner::new("".to_string()),
            parser          : Parser::new(),
        }
    }

    pub fn compile_from_path(&mut self, path_to_main : PathBuf) -> Result<Context, Error> {

        let mut main_code = "".to_string();

        let mut context = Context::new();

        if let Some(main) = std::fs::read_to_string(path_to_main).ok() {
            main_code = main;
            println!("{}", main_code);
        }

        self.scanner = Scanner::new(main_code);

        self.add_to_context(&mut context);
        Ok(context)
    }

    pub fn add_to_context(&mut self, ctx: &mut Context) {

        self.advance();

        while !self.matches(TokenType::Eof) {
            println!("{:?}", self.parser.current);
            self.advance();
        }
    }

    /// Advance one token
    fn advance(&mut self) {
        self.parser.previous = self.parser.current.clone();

        loop {
            self.parser.current = self.scanner.scan_token();

            if self.parser.current.kind != TokenType::Error {
                break;
            }
        }
    }

    /// If the current token matches advance, else do not.
    fn matches(&mut self, kind: TokenType) -> bool {
        if !self.check(kind) {
            false
        } else {
            self.advance();
            true
        }
    }

    /// Check if the current token matches.
    fn check(&self, kind: TokenType) -> bool {
        self.parser.current.kind == kind
    }

    /// get_The indent level of the current token.
    fn indent(&self) -> usize {
        self.parser.current.indent
    }
}