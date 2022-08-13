pub mod scanner;
pub mod context;
pub mod node;
pub mod object;

use crate::prelude::*;

use self::scanner::TokenType;

use super::element2d::texture::Texture;

#[derive(Clone, Debug)]
pub enum ErrorType {
    Syntax,
    FileNotFound,
}

#[derive(Clone, Debug)]
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
            // println!("{}", main_code);
        }

        self.scanner = Scanner::new(main_code);

        self.add_to_context(&mut context);

        if self.parser.error.is_some() {
            return Err(self.parser.error.clone().unwrap());
        }
        Ok(context)
    }

    pub fn add_to_context(&mut self, ctx: &mut Context) {

        self.advance();

        while !self.matches(TokenType::Eof) {
            //println!("{:?}", self.parser.current);

            let analytical = ["cube", "sphere"];

            if self.indent() == 0 {
                if self.parser.current.kind == TokenType::Identifier {
                    let idl = self.parser.current.lexeme.to_lowercase();
                    let id = idl.as_str();

                    if analytical.contains(&id){
                        self.object3d(ctx);
                    } else
                    if id == "texture" {
                        self.texture(ctx);
                    }
                }
            }

            self.advance();

            if self.parser.error.is_some() {
                break;
            }
        }
    }

    /// Reads an 3D object (analytical or SDF).
    fn object3d(&mut self, ctx: &mut Context) {
        let mut object : Option<Object> = None;
        if self.parser.current.lexeme == "Cube" {
            object = Some(Object::AnalyticalObject(Box::new(AnalyticalCube::new())));
        }
        if self.parser.current.lexeme == "Sphere" {
            object = Some(Object::AnalyticalObject(Box::new(AnalyticalSphere::new())));
        }

        self.advance();
        let mut is_root = false;

        if object.is_some() && self.check(TokenType::Star) {
            is_root = true;
            self.advance();
        }

        self.consume(TokenType::Less, "Expected '<' after object identifier.");
        // if self.check(TokenType::Less) == false {
        //     self.error_at_current("Expected '<' after object identifier.");
        // }

        //do {

        //} while self.parser.current.kind == TokenType::Comma;

        if self.parser.current.kind != TokenType::Greater {
            loop {
                let key = self.parser.current.lexeme.clone().to_lowercase();
                self.consume(TokenType::Identifier, "Expected identifier after '<'.");
                self.consume(TokenType::Colon, "Expected ':' after identifier.");

                let mut value = "".to_string();

                //let value = self.parser.current.lexeme.clone();
                //match self.parser.current.kind {
                //     TokenType::String => self.gcn().unwrap().add_property(name.clone(),
                //     Value::String(v.replace("\"", ""))),
                //     _ => { self.error_at_current(format!("Unknown property value for '{}'", name).as_str()); }
                // }

                while /* !self.check(TokenType::Comma) &&*/ !self.check(TokenType::Greater) && !self.check(TokenType::Eof) {
                    value += self.parser.current.lexeme.as_mut_str();
                    self.advance();
                }

                let code_blocks = ["onupdate".to_string()];

                if code_blocks.contains(&key) {
                    if let Some(object) = &mut object {
                        match object {
                            Object::AnalyticalObject(analytical) => {
                                //analytical.execute(code);
                                analytical.set_code_block(key.clone(), value.clone());
                            },
                            _ => {}
                        }
                    }
                } else {
                    let code = format!("let {} = {}", key, value);

                    if let Some(object) = &mut object {
                        match object {
                            Object::AnalyticalObject(analytical) => {
                                analytical.execute(code);
                                analytical.update();
                            },
                            _ => {}
                        }
                    }
                }

                println!("{:?}, {:?}", key, value);
                //self.advance();
                self.consume(TokenType::Greater, "Expected '>' after object properties.");
                //self.advance();

                if self.parser.current.kind != TokenType::Less {
                    break;
                } else {
                    self.advance();
                }
            }
        }
        //self.consume(TokenType::Greater, "Expected '>' after object properties.");

        if is_root {
            ctx.root.object = object.unwrap();
        }
    }

    /// Reads a texture
    fn texture(&mut self, ctx: &mut Context) {
        let mut object : Object = Object::Element2D(Box::new(Texture::new()));

        self.advance();
        let mut is_root = false;

        if self.check(TokenType::Star) {
            is_root = true;
            self.advance();
        }

        self.consume(TokenType::Less, "Expected '<' after object identifier.");

        if self.parser.current.kind != TokenType::Greater {
            loop {
                let key = self.parser.current.lexeme.clone().to_lowercase();
                self.consume(TokenType::Identifier, "Expected identifier after '<'.");
                self.consume(TokenType::Colon, "Expected ':' after identifier.");

                let mut value = "".to_string();

                while /* !self.check(TokenType::Comma) &&*/ !self.check(TokenType::Greater) && !self.check(TokenType::Eof) {
                    value += self.parser.current.lexeme.as_mut_str();
                    self.advance();
                }

                let code_blocks = ["onupdate".to_string()];

                if code_blocks.contains(&key) {
                    match &mut object {
                        Object::Element2D(element) => {
                            element.set_code_block(key.clone(), value.clone());
                        },
                        _ => {}
                    }

                } else {
                    let code = format!("let {} = {}", key, value);
                    match &mut object {
                        Object::Element2D(element) => {
                            element.execute(code);
                            //analytical.update();
                        },
                        _ => {}
                    }
                }

                println!("{:?}, {:?}", key, value);
                //self.advance();
                self.consume(TokenType::Greater, "Expected '>' after object properties.");

                if self.parser.current.kind != TokenType::Less {
                    break;
                } else {
                    self.advance();
                }
            }
        }
        //self.consume(TokenType::Greater, "Expected '>' after object properties.");

        match &mut object {
            Object::Element2D(texture) => {
                texture.alloc();
            },
            _ => {}
        }

        if is_root {
            ctx.root.object = object;
        } else {
            ctx.textures.push(object);
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

    /// Prints the current Token.
    fn debug_current(&mut self) {
        println!("Debug {:?}", self.parser.current);
    }

    /// Consume the current token if the type matches
    fn consume(&mut self, kind: TokenType, message: &str) {
        if self.parser.current.kind == kind {
            self.advance();
            return;
        }
        self.error_at_current(message);
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

    /// Error at the current token
    fn error_at_current(&mut self, message: &str) {
        self.error_at(self.parser.current.clone(), message)
    }

    /// Error at the previous token
    fn error(&mut self, message: &str) {
        self.error_at(self.parser.previous.clone(), message)
    }

    /// Error at the given token
    fn error_at(&mut self, _token: Token, message: &str) {
        if self.parser.error.is_some() { return; }
        self.parser.error = Some(Error::new(ErrorType::Syntax, message.to_string(), self.parser.current.line as u32));
    }
}