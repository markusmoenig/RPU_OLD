use crate::prelude::*;
pub struct ScriptEngine<'a> {
        engine              : Engine,
        scope               : Scope<'a>,

        code_blocks         : HashMap<String, String>
}

impl ScriptEngine<'_> {

    pub fn new() -> Self {

        let mut engine = Engine::new();

        engine.register_type_with_name::<F3>("F3")
            .register_fn("F3", F3::new_3)
            .register_fn("F3", F3::new_3)
            .register_get_set("x", F3::get_x, F3::set_x)
            .register_get_set("y", F3::get_y, F3::set_y)
            .register_get_set("z", F3::get_z, F3::set_z);


        Self {
            engine,
            scope           : Scope::new(),
            code_blocks     : HashMap::new(),
        }
    }

    pub fn get_scope<'a>(&mut self) -> &'a Scope {
        &mut self.scope
    }

    pub fn set_code_block(&mut self, name: String, code: String) {
        self.code_blocks.insert(name, code);
    }

    pub fn execute(&mut self, code: String) {
        let _rc = self.engine.eval_with_scope::<rhai::Dynamic>(&mut self.scope, code.as_str());
        //println!("{:?}", rc);
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

    pub fn get_vector3(&self, name: &str) -> Option<Vector3<F>> {
        if let Some(v) = self.scope.get_value::<F3>(name) {
            return Some(v.value);
        }
        None
    }

    pub fn set_vector3(&mut self, name: &str, v: Vector3<F>) {
        self.scope.set_value(name, F3::new(v));
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
