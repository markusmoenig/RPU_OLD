use crate::{prelude::*, rpu::compiler::ErrorType};
pub struct ScriptEngine<'a> {
        engine              : Engine,
        scope               : Scope<'a>,

        pub shader          : Option<AST>,

        code_blocks         : HashMap<String, String>
}

impl ScriptEngine<'_> {

    pub fn new() -> Self {

        let mut engine = Engine::new();

        engine.register_type_with_name::<F3>("F2")
            .register_fn("F2", F2::new_1)
            .register_fn("F2", F2::new_2)
            .register_get_set("x", F2::get_x, F2::set_x)
            .register_get_set("y", F2::get_y, F2::set_y);

        engine.register_type_with_name::<F3>("F3")
            .register_fn("F3", F3::new_1)
            .register_fn("F3", F3::new_3)
            .register_get_set("x", F3::get_x, F3::set_x)
            .register_get_set("y", F3::get_y, F3::set_y)
            .register_get_set("z", F3::get_z, F3::set_z);

        engine.register_type_with_name::<F4>("F4")
            .register_fn("F4", F4::new_1)
            .register_fn("F4", F4::new_4)
            .register_get_set("x", F4::get_x, F4::set_x)
            .register_get_set("y", F4::get_y, F4::set_y)
            .register_get_set("z", F4::get_z, F4::set_z)
            .register_get_set("w", F4::get_w, F4::set_w);

        engine.register_fn("length", |f2: F2| {
            f2.value.norm()
        });

        Self {
            engine,
            scope           : Scope::new(),
            code_blocks     : HashMap::new(),

            shader          : None,
        }
    }

    pub fn get_scope<'a>(&mut self) -> &'a Scope {
        &mut self.scope
    }

    pub fn set_code_block(&mut self, name: String, code: String) -> Result<(), RPUError> {

        if name == "shader" {

            let rc = self.engine.compile(code);

            if rc.is_ok() {
                if let Some(ast) = rc.ok() {
                    self.shader = Some(ast);
                }
            } else
            if let Some(error) = rc.err() {
                println!("{:?}", error.to_string());
                let err = RPUError::new(ErrorType::Syntax, error.to_string(), error.1.line().unwrap() as u32);
                return Err(err);
            }
        } else {
            self.code_blocks.insert(name, code);
        }

        Ok(())
    }

    pub fn execute(&mut self, code: String) {
        let _rc = self.engine.eval_with_scope::<rhai::Dynamic>(&mut self.scope, code.as_str());
        //println!("{:?}", rc);
    }

    pub fn execute_shader(&self, uv: &[F; 2]) -> Color {
        let mut color = [0.0, 0.0, 0.0, 1.0];

        let mut scope = Scope::new();
        scope.set_value("uv", F2::new_2(uv[0], uv[1]));

        if let Some(ast) = &self.shader {
            let rc = self.engine.eval_ast_with_scope::<F4>(&mut scope, &ast);

            if let Some(out) = rc.ok() {
                color[0] = out.value.x;
                color[1] = out.value.y;
                color[2] = out.value.z;
                color[3] = out.value.w;
            }
        }
        color
    }

    pub fn execute_block(&mut self, name: String) -> bool {

        if let Some(code) = &self.code_blocks.get(&name) {
            let rc = self.engine.eval_with_scope::<rhai::Dynamic>(&mut self.scope, code.as_str());

            if rc.is_ok() {
                return true;
            } else {
                println!("{:?}", rc);
            }
        }

        false
    }

    pub fn get_vector2(&self, name: &str) -> Option<Vector2<F>> {
        println!("{:?}", self.scope);
        if let Some(v) = self.scope.get_value::<F2>(name) {
            return Some(v.value);
        }
        None
    }

    pub fn set_vector2(&mut self, name: &str, v: Vector2<F>) {
        self.scope.set_value(name, F2::new(v));
    }

    pub fn get_vector3(&self, name: &str) -> Option<Vector3<F>> {
        if let Some(v) = self.scope.get_value::<F3>(name) {
            return Some(v.value);
        }
        None
    }

    pub fn set_vector3(&mut self, name: &str, v: Vector3<F>) {
        self.scope.set_value(name, F3::new(v));
    }

    pub fn get_vector4(&self, name: &str) -> Option<Vector4<F>> {
        if let Some(v) = self.scope.get_value::<F4>(name) {
            return Some(v.value);
        }
        None
    }

    pub fn set_vector4(&mut self, name: &str, v: Vector4<F>) {
        self.scope.set_value(name, F4::new(v));
    }

    pub fn get_float(&self, name: &str) -> Option<F> {
        if let Some(v) = self.scope.get_value::<F>(name) {
            return Some(v);
        }
        None
    }

    pub fn set_float(&mut self, name: &str, v: F) {
        self.scope.set_value(name, v);
    }

    pub fn get_int(&self, name: &str) -> Option<I> {
        if let Some(v) = self.scope.get_value::<I>(name) {
            return Some(v);
        }
        None
    }

    pub fn set_int(&mut self, name: &str, v: I) {
        self.scope.set_value(name, v);
    }

    pub fn get_string(&self, name: &str) -> Option<String> {
        if let Some(v) = self.scope.get_value::<String>(name) {
            return Some(v);
        }
        None
    }
}
