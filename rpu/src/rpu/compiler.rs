pub mod scanner;
pub mod context;
pub mod node;
pub mod object;

use crate::prelude::*;

use self::scanner::TokenType;

#[derive(Clone, Debug)]
pub enum ErrorType {
    Syntax,
    FileNotFound,
}

#[derive(Clone, Debug)]
pub struct RPUError {
    error_type              : ErrorType,
    description             : String,
    line                    : u32,
}

impl RPUError {
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

    error               : Option<RPUError>,
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

pub enum Property {
    Property(String, String),
    Function(String, String, String),
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

    pub fn compile_from_path(&mut self, path_to_main : PathBuf) -> Result<Context, RPUError> {

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

            let camera3d = ["pinhole"];
            let objects3d = ["voxel", "sdfcube", "sdfsphere"];
            let layouts = ["grid3d"];
            let mut consumed = false;

            if self.indent() == 0 {

                if self.parser.current.kind == TokenType::Identifier {
                    let idl = self.parser.current.lexeme.to_lowercase();
                    let id = idl.as_str();

                    if camera3d.contains(&id){
                        self.camera3d(ctx);
                        consumed = true;
                    } else
                    if objects3d.contains(&id){
                        self.object3d(ctx);
                        consumed = true;
                    } else
                    if layouts.contains(&id) {
                        self.layout3d(ctx);
                        consumed = true;
                    } else
                    if id == "texture" {
                        self.texture(ctx);
                        consumed = true;
                    }
                }
            }

            if consumed == false {
                self.advance();
            }

            if self.parser.error.is_some() {
                break;
            }
        }
    }

    /// Reads an 3D object (analytical or SDF).
    fn object3d(&mut self, ctx: &mut Context) {

        let mut object : Option<Object> = None;
        let mut symbol : Option<char> = None;

        self.debug_current();
        // if self.parser.current.lexeme == "Cube" {
        //     object = Some(Object::AnalyticalObject(Box::new(AnalyticalCube::new())));
        // }
        if self.parser.current.lexeme.to_lowercase() == "voxel" {
            object = Some(Object::AnalyticalObject(Box::new(AnalyticalVoxel::new())));
        } else
        if self.parser.current.lexeme.to_lowercase() == "sdfcube" {
            object = Some(Object::SDF3D(Box::new(SDF3DCube::new())));
        } else
        if self.parser.current.lexeme.to_lowercase() == "sdfsphere" {
            object = Some(Object::SDF3D(Box::new(SDF3DSphere::new())));
        }

        self.advance();

        if object.is_some() && self.check(TokenType::Apostrophe) {
            self.advance();
            let c = self.parser.current.lexeme.chars().next();
            if let Some(c) = c {
                symbol = Some(c);
                self.advance();
            }
        }

        //self.consume(TokenType::Less, "Expected '<' after object identifier.");

        let props = self.parse_object_properties();

        if let Some(object) = &mut object {
            match object {
                Object::AnalyticalObject(object) => {
                    self.parser.error = object.apply_properties(props).err();
                },
                Object::SDF3D(sdf) => {
                    self.parser.error = sdf.apply_properties(props).err();
                },
                _ => {},
            }
        }

        // Get the texture name if any
        let mut texture : Option<usize> = None;

        if let Some(object) = &object {
            match object {
                Object::AnalyticalObject(object) => {
                    if let Some(name) = object.get_engine().get_string("texture") {
                        texture = self.get_texture_index(name, ctx);
                    }
                },
                Object::SDF3D(sdf) => {
                    if let Some(name) = sdf.get_engine().get_string("texture") {
                        texture = self.get_texture_index(name, ctx);
                    }
                },
                _ => {}
            }
        }

        let mut node = Node::new();
        node.object = object.unwrap();
        node.texture = texture;

        if let Some(symbol) = symbol {
            ctx.symbols_node_index.insert(symbol, ctx.nodes.len());
        }
        ctx.nodes.push(node);
    }

    /// Reads a 3d layout.
    fn layout3d(&mut self, ctx: &mut Context) {

        let mut object : Option<Object> = None;
        let mut symbol : Option<char> = None;

        // if self.parser.current.lexeme.to_lowercase() == "grid2d" {
        //     object = Some(Object::Layout3D(Box::new(Grid2D::new())));
        // } else
        if self.parser.current.lexeme.to_lowercase() == "grid3d" {
            object = Some(Object::Layout3D(Box::new(Grid3D::new())));
        }

        self.advance();

        if object.is_some() && self.check(TokenType::Apostrophe) {
            self.advance();
            let c = self.parser.current.lexeme.chars().next();
            if let Some(c) = c {
                symbol = Some(c);
                self.advance();
            }
        }

        //self.consume(TokenType::Less, "Expected '<' after object identifier.");

        if let Some(object) = &mut object {
            //self.parse_object_properties(object);

            let mut map : HashMap<(i32, i32, i32), usize> = HashMap::new();

            let mut x = 0;
            let y = 0;
            let mut z = 0;

            let mut first = true;

            loop {
                if self.check(TokenType::Colon) {

                    while self.check(TokenType::Colon) {
                        // Next line
                        if first == true {
                            first = false;
                        } else {
                            z += 1;
                            x = 0;
                        }
                        self.advance_with_whitespace();
                    }
                    //self.consume_with_whitespace(TokenType::Space, "Expect ' ' after colon.");
                } else
                if self.check(TokenType::Space) {
                    x += 1;
                    self.advance_with_whitespace();
                } else
                if self.check(TokenType::Identifier) != true {
                    break;
                }

                let symbols = self.parser.current.lexeme.chars();
                //println!("{:?}", symbols);

                for c in symbols {
                    if c == ' ' {
                        x += 1;
                    } else
                    if let Some(index) = ctx.symbols_node_index.get(&c) {
                        map.insert((x, y, z), *index);
                        x+= 1;
                    } else {
                        self.error_at_current(format!("Undefined instance symbol '{}'.", c).as_str());
                        break;
                    }
                }

                self.advance_with_whitespace();
            }

            //println!("{:?}", map);

            match object {
                Object::Layout3D(layout) => layout.set_map3d(map),
                _ => {}
            }
        }

        if let Some(symbol) = symbol {
            ctx.symbols_node_index.insert(symbol, ctx.nodes.len());
        }
        ctx.layouts.push(object.unwrap());
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

        let props = self.parse_object_properties();

        match &mut object {
            Object::Element2D(texture) => {
                self.parser.error = texture.apply_properties(props).err();
                texture.render();
            },
            _ => {}
        }

        if is_root {
            let mut node = Node::new();
            node.object = object;
            ctx.nodes.push(node);
        } else {
            ctx.textures.push(object);
        }
    }

    /// Reads a camera
    fn camera3d(&mut self, ctx: &mut Context) {
        let mut object = Box::new(Pinhole::new());

        self.advance();

        let props = self.parse_object_properties();
        self.parser.error = object.apply_properties(props).err();

        ctx.camera = object;
    }

    /// Returns the index of the texture with the given name
    fn get_texture_index(&self, name: String, ctx: &mut Context) -> Option<usize> {
        for (index, object) in ctx.textures.iter().enumerate() {
            match object {
                Object::Element2D(el) => {
                    //texture.alloc();
                    if let Some(rc) = el.get_engine().get_string("name") {
                        if rc == name {
                            return Some(index)
                        }
                    }
                },
                _ => {}
            }
        }
        None
    }

    /// Parses the properties for the given object
    fn parse_object_properties(&mut self) -> Vec<Property> {

        //let object_line = self.parser.current.line;
        //println!("object on line {}", object_line);

        let mut props : Vec<Property> = vec![];

        loop {
            let property = self.parser.current.lexeme.clone();
            let indention = self.parser.current.indent;
            //let line = self.parser.current.line;
            self.consume(TokenType::Identifier, "Expected identifier.");

            if self.check(TokenType::Equal) {
                let value = self.scanner.scanline(1);
                //println!("assignment, line {}: {} = {}", line, property, value);
                props.push(Property::Property(property, value));
                self.advance();
                if self.indent() == 0 {
                    break;
                }
            } else
            if self.check(TokenType::LeftParen) {
                let mut args = "".to_string();
                self.advance();
                loop {
                    if self.check(TokenType::Identifier) {
                        args += self.parser.current.lexeme.clone().as_str();
                        self.advance();
                    } else
                    if self.check(TokenType::RightParen) {
                        break;
                    } else
                    if self.check(TokenType::Comma) {
                        args += ",";
                        self.advance();
                    } else {
                        self.error_at_current("Invalid function arguments");
                        break;
                    }
                }
                let code = self.scanner.scan_indention_block(1, indention);
                //println!("function, line {}: {}, {:?}", line, args, code.ok());
                if let Some(code) = code.ok() {
                    props.push(Property::Function(property, args, code));
                }
                self.advance();
                if self.indent() == 0 {
                    break;
                }
                self.debug_current();
            } else {
                break;
            }
        }

        props

        /*
        if self.parser.current.kind != TokenType::Greater {
            loop {
                let key = self.parser.current.lexeme.clone().to_lowercase();

                if self.check(TokenType::Slash) == true && self.scanner.peek() == b'>' {
                    self.advance();
                    self.advance();
                    break;
                }

                self.consume(TokenType::Identifier, "Expected identifier after '<'.");
                self.consume(TokenType::Colon, "Expected ':' after identifier.");

                let mut value = "".to_string();

                while (self.check(TokenType::Slash) == false && self.scanner.peek() != b'>') && !self.check(TokenType::Eof) {
                    value += self.parser.current.lexeme.as_mut_str();
                    self.advance_with_whitespace();
                }

                let code_blocks = ["update".to_string(), "shader".to_string()];

                if code_blocks.contains(&key) {
                    match object {
                        Object::AnalyticalObject(analytical) => {
                            analytical.set_code_block(key.clone(), value.clone());
                        },
                        Object::SDF3D(sdf) => {
                            sdf.set_code_block(key.clone(), value.clone());
                        },
                        Object::Layout3D(layout) => {
                            layout.set_code_block(key.clone(), value.clone());
                        },
                        Object::Element2D(element) => {
                            element.set_code_block(key.clone(), value.clone());
                        },
                        Object::Camera3D(camera) => {
                            camera.set_code_block(key.clone(), value.clone());
                        },
                        _ => {}
                    }
                } else {
                    let code = format!("let {} = {}", key, value);
                    match object {
                        Object::AnalyticalObject(analytical) => {
                            analytical.execute(code);
                            analytical.update();
                        },
                        Object::SDF3D(sdf) => {
                            sdf.execute(code);
                        },
                        Object::Layout3D(layout) => {
                            layout.execute(code);
                        },
                        Object::Element2D(element) => {
                            element.execute(code);
                        },
                        Object::Camera3D(camera) => {
                            camera.execute(code);
                        },
                        _ => {}
                    }
                }

                println!("{:?}, {:?}", key, value);
                self.consume(TokenType::Slash, "Expected '/>' after object properties.");
                self.consume(TokenType::Greater, "Expected '/>' after object properties.");

                if self.parser.current.kind != TokenType::Less || self.parser.current.indent == 0 {
                    break;
                } else {
                    self.advance();
                }
            }
        } else {
            self.advance();
        }*/
    }

    /// Advance one token
    fn advance(&mut self) {
        self.parser.previous = self.parser.current.clone();

        loop {
            self.parser.current = self.scanner.scan_token(false);

            if self.parser.current.kind != TokenType::Error {
                break;
            }
        }
    }

    /// Advance one token and allow whitespace
    fn advance_with_whitespace(&mut self) {
        self.parser.previous = self.parser.current.clone();

        loop {
            self.parser.current = self.scanner.scan_token(true);

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

    /// Consume the current token if the type matches
    fn _consume_with_whitespace(&mut self, kind: TokenType, message: &str) {
        if self.parser.current.kind == kind {
            self.advance_with_whitespace();
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
    fn _error(&mut self, message: &str) {
        self.error_at(self.parser.previous.clone(), message)
    }

    /// Error at the given token
    fn error_at(&mut self, _token: Token, message: &str) {
        if self.parser.error.is_some() { return; }
        self.parser.error = Some(RPUError::new(ErrorType::Syntax, message.to_string(), self.parser.current.line as u32));
    }
}