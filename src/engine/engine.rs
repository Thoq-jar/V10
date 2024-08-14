use std::fs;
use boa_engine::{Context, Source, JsResult};
use crate::utils::helper::register_console;

pub struct Engine {
    state: String,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            state: String::from("initialized"),
        }
    }

    pub fn run(&self) {
        println!("[V12]: Engine running with state: {}", self.state);
    }

    pub fn interpret_js(&self, script_path: &str) -> JsResult<()> {
        let script = fs::read_to_string(script_path).expect(&format!("[V12]: Unable to read file: {}", script_path));
        let mut context = Context::default();

        register_console(&mut context);
        context.eval(Source::from_bytes(&script))?;

        Ok(())
    }
}