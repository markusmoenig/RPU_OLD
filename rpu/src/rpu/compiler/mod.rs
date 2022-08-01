pub mod scanner;
pub mod context;

use crate::prelude::*;

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

pub struct Compiler {
}

impl Compiler {

    pub fn new() -> Self {
        Self {
        }
    }

    pub fn compile_from_path(path_to_main : PathBuf) -> Result<Context, Error> {

        let mut context = Context::new();

        let mut main_code = "".to_string();

        if let Some(main) = std::fs::read_to_string(path_to_main).ok() {
            main_code = main;
            println!("{}", main_code);
        }

        Ok(context)
    }
}