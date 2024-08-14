use crate::utils::helper::register_console;
use boa_engine::{Context, JsResult, Source};
use std::fs;

pub struct Engine {
  pub(crate) state: String,
}

impl Engine {
  pub fn new() -> Self {
    Engine {
      state: String::from("initialized"),
    }
  }

  pub fn run(&self) {
    self.interpret_js("src/js/V12.js")
      .expect("[V12]: Unable to start the engine!");
    println!("[V12]: Engine running with state: {}", self.state);
  }

  pub fn interpret_js(&self, script_path: &str) -> JsResult<()> {
    let script: String = fs::read_to_string(script_path)
      .expect(&format!("[V12]: Unable to read file: {}", script_path));
    let mut context: Context = Context::default();

    register_console(&mut context);
    context.eval(Source::from_bytes(&script))?;

    Ok(())
  }
}
