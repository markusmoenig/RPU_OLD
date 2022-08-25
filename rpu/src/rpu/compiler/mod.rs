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
    pub error_type              : ErrorType,
    pub description             : String,
    pub line                    : u32,
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

    elements2d              : Vec<String>,
    objects3d               : Vec<String>,

    curr_parent             : Option<usize>
}

impl Compiler {

    pub fn new() -> Self {
        Self {
            scanner         : Scanner::new("".to_string()),
            parser          : Parser::new(),

            elements2d      : vec!["Texture".to_string(), "Vertical".to_string(), "Color".to_string(), "Noise".to_string(), "Bricks"
            .to_string()],
            objects3d       : vec!["Voxel".to_string(), "sdfCube".to_string(), "sdfSphere".to_string()],

            curr_parent     : None,
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

        self.curr_parent = None;
        self.add_to_context(&mut context);

        if self.parser.error.is_some() {
            return Err(self.parser.error.clone().unwrap());
        }

        // Initialize the context, like rendering static textures etc.
        context.init();

        // Log the project structure

        struct StructureEnv { indent: u32 }
        fn log_node (node_index: usize, env: &mut StructureEnv, ctx: &Context) {
            let mut message = "".to_string();
            for _ in 0..env.indent {
                message += " ";
            }
            log::info!("{}{:?}, Elements: {}", message, ctx.nodes[node_index].get_node_type(), ctx.nodes[node_index].elements.len());
            env.indent += 2;
            for n in &ctx.nodes[node_index].childs {
                log_node(*n, env, ctx);
            }
            env.indent -= 2;
        }

        log::info!("Textures ---------------");

        for n in &context.textures {
            log_node(*n, &mut StructureEnv { indent: 0 }, &context);
        }

        log::info!("Objects ---------------");

        for n in &context.objects {
            log_node(*n, &mut StructureEnv { indent: 0 }, &context);
        }

        Ok(context)
    }

    /// Logger
    pub fn log(&self, msg: String) {
        let mut message = "".to_string();
        for _ in 0..self.parser.current.indent {
            message += " ";
        }
        message += msg.as_str();
        message += format!(" on line {}", self.parser.current.line).as_str();
        log::info!("{}", message);
    }

    pub fn add_to_context(&mut self, ctx: &mut Context) {

        self.advance();

        while !self.matches(TokenType::Eof) {

            let camera3d = ["Pinhole"];
            let layouts = ["Grid3D"];
            let mut consumed = false;

            if self.indent() == 0 {

                if self.parser.current.kind == TokenType::Identifier {
                    let idl = self.parser.current.lexeme.clone();
                    let id = idl.as_str();

                    if camera3d.contains(&id){
                        self.log(format!("Camera ({})", self.parser.current.lexeme));
                        self.camera3d(ctx);
                        consumed = true;
                    } else
                    if self.objects3d.contains(&id.to_string()){
                        self.log(format!("Object3D ({})", self.parser.current.lexeme));
                        self.object3d(ctx);
                        consumed = true;
                    } else
                    if layouts.contains(&id) {
                        self.log(format!("Layout3D ({})", self.parser.current.lexeme));
                        self.layout3d(ctx);
                        consumed = true;
                    } else
                    if id == "Texture" {
                        self.log(format!("Element2D ({})", self.parser.current.lexeme));
                        self.element2d(ctx);
                        consumed = true;
                    } else
                    if id == "Sprite" {
                        self.log(format!("Sprite ({})", self.parser.current.lexeme));
                        self.sprite(ctx);
                        consumed = true;
                    }
                }
            } else {
                if self.parser.current.kind == TokenType::Identifier {
                    let idl = self.parser.current.lexeme.clone();

                    if self.elements2d.contains(&idl) {
                        self.log(format!("Element2D ({})", self.parser.current.lexeme));
                        self.element2d(ctx);
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

        // if self.parser.current.lexeme == "Cube" {
        //     object = Some(Object::AnalyticalObject(Box::new(AnalyticalCube::new())));
        // }
        if self.parser.current.lexeme == "Voxel" {
            object = Some(Object::AnalyticalObject(Box::new(AnalyticalVoxel::new())));
        } else
        if self.parser.current.lexeme == "sdfCube" {
            object = Some(Object::SDF3D(Box::new(SDF3DCube::new())));
        } else
        if self.parser.current.lexeme == "sdfSphere" {
            object = Some(Object::SDF3D(Box::new(SDF3DSphere::new())));
        }

        let mut node = Node::new(format!("{}, line {}", self.parser.current.lexeme, self.parser.current.line));

        self.advance();

        if object.is_some() && self.check(TokenType::Apostrophe) {
            self.advance();
            let c = self.parser.current.lexeme.chars().next();
            if let Some(c) = c {
                symbol = Some(c);
                self.advance();
            }
        }

        node.object = object.unwrap();

        let props = self.parse_object_properties(&mut node);

        match &mut node.object {
            Object::AnalyticalObject(object) => {
                self.parser.error = object.apply_properties(props).err();
            },
            Object::SDF3D(sdf) => {
                self.parser.error = sdf.apply_properties(props).err();
            },
            _ => {},
        }

        // Get the texture name if any
        let mut texture : Option<usize> = None;

        match &mut node.object {
            Object::AnalyticalObject(object) => {
                if let Some(name) = object.get_engine().get_string("texture") {
                    texture = self.get_texture_index(name.clone(), ctx);
                    println!("result {}, {:?}", name, texture);
                }
            },
            Object::SDF3D(sdf) => {
                if let Some(name) = sdf.get_engine().get_string("texture") {
                    texture = self.get_texture_index(name.clone(), ctx);
                    println!("result {}, {:?}", name, texture);
                }
            },
            _ => {}
        }

        node.texture = texture;

        if let Some(symbol) = symbol {
            ctx.symbols_node_index.insert(symbol, ctx.nodes.len());
        }

        ctx.objects.push(ctx.nodes.len());
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

        if let Some(object) = &mut object {
            let mut map : HashMap<(i32, i32, i32), usize> = HashMap::new();

            let mut x = 0;
            let mut y = 0;
            let mut z = 0;

            let mut first = true;

            loop {
                if self.check(TokenType::Colon) {

                    let line = self.parser.current.line;
                    let mut count = 0;

                    while self.check(TokenType::Colon) {

                        // Next level up
                        if count > 0 && self.parser.current.line == line {
                            y += 1;
                            x = 0;
                            z = 0;
                        }

                        // Next line
                        if first == true {
                            first = false;
                        } else {
                            z += 1;
                            x = 0;
                        }
                        self.advance_with_whitespace();
                        count += 1;
                    }
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

    /// Reads a texture or 2d element
    fn element2d(&mut self, ctx: &mut Context) {
        let mut object : Option<Object> = None;

        let mut is_layout = false;

        if self.parser.current.lexeme == "Texture" {
            is_layout = true;
            object = Some(Object::Element2D(Box::new(Texture::new())));
        } else
        if self.parser.current.lexeme == "Vertical" {
            is_layout = true;
            object = Some(Object::Element2D(Box::new(Vertical::new())));
        } else
        if self.parser.current.lexeme == "Color" {
            object = Some(Object::Element2D(Box::new(ColorElement::new())));
        } else
        if self.parser.current.lexeme == "Noise" {
            object = Some(Object::Element2D(Box::new(Noise::new())));
        } else
        if self.parser.current.lexeme == "Bricks" {
            object = Some(Object::Element2D(Box::new(Bricks::new())));
        }

        let mut node = Node::new(format!("{}, line {}", self.parser.current.lexeme, self.parser.current.line));
        node.object = object.unwrap();

        if self.parser.current.indent == 0 {
            self.curr_parent = None;
        }

        // if let Some(parent_index) = self.curr_parent {
        //     if ctx.nodes[parent_index].indent < self.parser.current.indent {
        //         self.curr_parent = None;
        //     }
        // }

        self.advance();

        let props = self.parse_object_properties(&mut node);

        match &mut node.object {
            Object::Element2D(texture) => {
                self.parser.error = texture.apply_properties(props).err();
            },
            _ => {}
        }

        if is_layout {
            let index = ctx.nodes.len();
            ctx.nodes.push(node);
            if let Some(parent_index) = self.curr_parent {
                ctx.nodes[parent_index].childs.push(index);
            } else {
                ctx.textures.push(index);
                println!("Added as texture: {}", self.get_name_of_object(&ctx.nodes[index].object));
            }
            self.curr_parent = Some(index);
        } else
        if let Some(parent_index) = self.curr_parent {
            let index = ctx.nodes.len();
            ctx.nodes.push(node);
            ctx.nodes[parent_index].elements.push(index);
            println!("Added {}({}) to: {}({})", self.get_name_of_object(&ctx.nodes[index].object), index, self.get_name_of_object(&ctx.nodes[parent_index].object), parent_index);
        }
        //self.debug_current("end of texture");
    }

    /// Reads a sprite, a special case Element2D
    fn sprite(&mut self, ctx: &mut Context) {
        let mut object = Box::new(Sprite::new());

        let mut node = Node::new(format!("{}, line {}", self.parser.current.lexeme, self.parser.current.line));

        self.advance();

        let props = self.parse_object_properties(&mut node);
        self.parser.error = object.apply_properties(props).err();

        let mut texture : Option<usize> = None;
        if let Some(name) = object.get_engine().get_string("texture") {
            texture = self.get_texture_index(name.clone(), ctx);
        }
        object.texture = texture;
        ctx.sprites.push(object);
    }

    /// Reads a camera
    fn camera3d(&mut self, ctx: &mut Context) {
        let mut object = Box::new(Pinhole::new());

        let mut node = Node::new(format!("{}, line {}", self.parser.current.lexeme, self.parser.current.line));

        self.advance();

        let props = self.parse_object_properties(&mut node);
        self.parser.error = object.apply_properties(props).err();

        ctx.camera = object;
    }

    /// Returns the index of the texture with the given name
    fn get_texture_index(&self, name: String, ctx: &mut Context) -> Option<usize> {
        for (index, node_index) in ctx.textures.iter().enumerate() {
            match &ctx.nodes[*node_index].object {
                Object::Element2D(el) => {
                    //texture.alloc();
                    if let Some(rc) = el.get_engine().get_string("name") {
                        if rc == name {
                            return Some(ctx.textures[index])
                        }
                    }
                },
                _ => {}
            }
        }
        None
    }

    /// Parses the properties for the given object
    fn parse_object_properties(&mut self, node: &mut Node) -> Vec<Property> {

        node.indent = self.parser.current.indent;
        //println!("object on line {}", self.parser.current.line);

        let mut props : Vec<Property> = vec![];

        loop {
            let property = self.parser.current.lexeme.clone();
            let indention = self.parser.current.indent;

            if self.elements2d.contains(&property) || self.objects3d.contains(&property) || self.parser.current.indent < node.indent {
                self.debug_current(format!("prop break for {}", node.id).as_str());
                break;
            }

            self.consume(TokenType::Identifier, "Expected identifier.");

            if self.check(TokenType::Equal) {
                let value = self.scanner.scanline(1);
                println!("assignment to {:?}, line {}: {} = {}", node.id, self.parser.current.line, property, value);
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
                if self.indent() <= node.indent {
                    break;
                }
            } else {
                break;
            }
        }
        props
    }

    /// Returns the name of an object
    fn get_name_of_object(&self, object: &Object) -> String {
        match object {
            Object::Element2D(el) => {
                el.name()
            },
            _ => { "??".to_string() }
        }
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
    fn debug_current(&mut self, msg: &str) {
        println!("{} {:?}", msg, self.parser.current);
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